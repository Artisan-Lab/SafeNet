pub fn device_create_buffer<A: HalApi>(
    &self,
    device_id: DeviceId,
    desc: &resource::BufferDescriptor,
    id_in: Input<G, id::BufferId>,
) -> (id::BufferId, Option<resource::CreateBufferError>) {
    profiling::scope!("Device::create_buffer");

    let hub = A::hub(self);
    let mut token = Token::root();
    let fid = hub.buffers.prepare(id_in);

    let (device_guard, mut token) = hub.devices.read(&mut token);
    let error = loop {
        let device = match device_guard.get(device_id) {
            Ok(device) => device,
            Err(_) => break DeviceError::Invalid.into(),
        };
        if !device.valid {
            break DeviceError::Invalid.into();
        }

        if desc.usage.is_empty() {
            // Per spec, `usage` must not be zero.
            break resource::CreateBufferError::InvalidUsage(desc.usage);
        }

        #[cfg(feature = "trace")]
        if let Some(ref trace) = device.trace {
            let mut desc = desc.clone();
            let mapped_at_creation = mem::replace(&mut desc.mapped_at_creation, false);
            if mapped_at_creation && !desc.usage.contains(wgt::BufferUsages::MAP_WRITE) {
                desc.usage |= wgt::BufferUsages::COPY_DST;
            }
            trace
                .lock()
                .add(trace::Action::CreateBuffer(fid.id(), desc));
        }

        let mut buffer = match device.create_buffer(device_id, desc, false) {
            Ok(buffer) => buffer,
            Err(e) => break e,
        };
        let ref_count = buffer.life_guard.add_ref();

        let buffer_use = if !desc.mapped_at_creation {
            hal::BufferUses::empty()
        } else if desc.usage.contains(wgt::BufferUsages::MAP_WRITE) {
            // buffer is mappable, so we are just doing that at start
            let map_size = buffer.size;
            let ptr = if map_size == 0 {
                std::ptr::NonNull::dangling()
            } else {
                match map_buffer(&device.raw, &mut buffer, 0, map_size, HostMap::Write) {
                    Ok(ptr) => ptr,
                    Err(e) => {
                        let raw = buffer.raw.unwrap();
                        device.lock_life(&mut token).schedule_resource_destruction(
                            queue::TempResource::Buffer(raw),
                            !0,
                        );
                        break e.into();
                    }
                }
            };
            buffer.map_state = resource::BufferMapState::Active {
                ptr,
                range: 0..map_size,
                host: HostMap::Write,
            };
            hal::BufferUses::MAP_WRITE
        } else {
            // buffer needs staging area for initialization only
            let stage_desc = wgt::BufferDescriptor {
                label: Some(Cow::Borrowed(
                    "(wgpu internal) initializing unmappable buffer",
                )),
                size: desc.size,
                usage: wgt::BufferUsages::MAP_WRITE | wgt::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            };
            let mut stage = match device.create_buffer(device_id, &stage_desc, true) {
                Ok(stage) => stage,
                Err(e) => {
                    let raw = buffer.raw.unwrap();
                    device
                        .lock_life(&mut token)
                        .schedule_resource_destruction(queue::TempResource::Buffer(raw), !0);
                    break e;
                }
            };
            let stage_buffer = stage.raw.unwrap();
            let mapping = match unsafe { device.raw.map_buffer(&stage_buffer, 0..stage.size) } {
                Ok(mapping) => mapping,
                Err(e) => {
                    let raw = buffer.raw.unwrap();
                    let mut life_lock = device.lock_life(&mut token);
                    life_lock
                        .schedule_resource_destruction(queue::TempResource::Buffer(raw), !0);
                    life_lock.schedule_resource_destruction(
                        queue::TempResource::Buffer(stage_buffer),
                        !0,
                    );
                    break DeviceError::from(e).into();
                }
            };

            assert_eq!(buffer.size % wgt::COPY_BUFFER_ALIGNMENT, 0);
            // Zero initialize memory and then mark both staging and buffer as initialized
            // (it's guaranteed that this is the case by the time the buffer is usable)
            unsafe { ptr::write_bytes(mapping.ptr.as_ptr(), 0, buffer.size as usize) };
            buffer.initialization_status.drain(0..buffer.size);
            stage.initialization_status.drain(0..buffer.size);

            buffer.map_state = resource::BufferMapState::Init {
                ptr: mapping.ptr,
                needs_flush: !mapping.is_coherent,
                stage_buffer,
            };
            hal::BufferUses::COPY_DST
        };

        let id = fid.assign(buffer, &mut token);
        log::trace!("Device::create_buffer -> {:?}", id.0);

        device
            .trackers
            .lock()
            .buffers
            .insert_single(id, ref_count, buffer_use);

        return (id.0, None);
    };

    let id = fid.assign_error(desc.label.borrow_or_default(), &mut token);
    (id, Some(error))
}