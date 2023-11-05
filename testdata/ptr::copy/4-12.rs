pub fn execute_async(
    &mut self,
    mem: &GuestMemoryMmap,
    disk_nsectors: u64,
    disk_image: &mut dyn AsyncIo,
    disk_id: &[u8],
    user_data: u64,
) -> result::Result<bool, ExecuteError> {
    let sector = self.sector;
    let request_type = self.request_type;
    let offset = (sector << SECTOR_SHIFT) as libc::off_t;

    let mut iovecs: SmallVec<[libc::iovec; 1]> =
        SmallVec::with_capacity(self.data_descriptors.len());
    for (data_addr, data_len) in &self.data_descriptors {
        if *data_len == 0 {
            continue;
        }
        let mut top: u64 = u64::from(*data_len) / SECTOR_SIZE;
        if u64::from(*data_len) % SECTOR_SIZE != 0 {
            top += 1;
        }
        top = top
            .checked_add(sector)
            .ok_or(ExecuteError::BadRequest(Error::InvalidOffset))?;
        if top > disk_nsectors {
            return Err(ExecuteError::BadRequest(Error::InvalidOffset));
        }

        let origin_ptr = mem
            .get_slice(*data_addr, *data_len as usize)
            .map_err(ExecuteError::GetHostAddress)?
            .as_ptr();

        // Verify the buffer alignment.
        // In case it's not properly aligned, an intermediate buffer is
        // created with the correct alignment, and a copy from/to the
        // origin buffer is performed, depending on the type of operation.
        let iov_base = if (origin_ptr as u64) % SECTOR_SIZE != 0 {
            let layout =
                Layout::from_size_align(*data_len as usize, SECTOR_SIZE as usize).unwrap();
            // SAFETY: layout has non-zero size
            let aligned_ptr = unsafe { alloc_zeroed(layout) };
            if aligned_ptr.is_null() {
                return Err(ExecuteError::TemporaryBufferAllocation(
                    io::Error::last_os_error(),
                ));
            }

            // We need to perform the copy beforehand in case we're writing
            // data out.
            if request_type == RequestType::Out {
                // SAFETY: destination buffer has been allocated with
                // the proper size.
                unsafe {
                    std::ptr::copy(origin_ptr as *const u8, aligned_ptr, *data_len as usize)
                };
            }

            // Store both origin and aligned pointers for complete_async()
            // to process them.
            self.aligned_operations.push(AlignedOperation {
                origin_ptr: origin_ptr as u64,
                aligned_ptr: aligned_ptr as u64,
                size: *data_len as usize,
                layout,
            });

            aligned_ptr as *mut libc::c_void
        } else {
            origin_ptr as *mut libc::c_void
        };

        let iovec = libc::iovec {
            iov_base,
            iov_len: *data_len as libc::size_t,
        };
        iovecs.push(iovec);
    }

    // Queue operations expected to be submitted.
    match request_type {
        RequestType::In => {
            for (data_addr, data_len) in &self.data_descriptors {
                mem.get_slice(*data_addr, *data_len as usize)
                    .map_err(ExecuteError::GetHostAddress)?
                    .bitmap()
                    .mark_dirty(0, *data_len as usize);
            }
            disk_image
                .read_vectored(offset, &iovecs, user_data)
                .map_err(ExecuteError::AsyncRead)?;
        }
        RequestType::Out => {
            disk_image
                .write_vectored(offset, &iovecs, user_data)
                .map_err(ExecuteError::AsyncWrite)?;
        }
        RequestType::Flush => {
            disk_image
                .fsync(Some(user_data))
                .map_err(ExecuteError::AsyncFlush)?;
        }
        RequestType::GetDeviceId => {
            let (data_addr, data_len) = if self.data_descriptors.len() == 1 {
                (self.data_descriptors[0].0, self.data_descriptors[0].1)
            } else {
                return Err(ExecuteError::BadRequest(Error::TooManyDescriptors));
            };
            if (data_len as usize) < disk_id.len() {
                return Err(ExecuteError::BadRequest(Error::InvalidOffset));
            }
            mem.write_slice(disk_id, data_addr)
                .map_err(ExecuteError::Write)?;
            return Ok(false);
        }
        RequestType::Unsupported(t) => return Err(ExecuteError::Unsupported(t)),
    }

    Ok(true)
}
