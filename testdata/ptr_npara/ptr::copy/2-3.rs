pub fn update_with_f32_data(&mut self, metal_cx: &MetalCx, data: &Vec<f32>) {
    let elem = self.multi_buffer_write();
    if elem.size < data.len() {
        elem.buffer = None;
    }
    if let None = &elem.buffer {
        elem.buffer = Some(
            metal_cx.device.new_buffer(
                (data.len() * std::mem::size_of::<f32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            )
        );
        elem.size = data.len()
    }
    if let Some(buffer) = &elem.buffer {
        let p = buffer.contents();
        
        unsafe {
            std::ptr::copy(data.as_ptr(), p as *mut f32, data.len());
        }
        buffer.did_modify_range(NSRange::new(0 as u64, (data.len() * std::mem::size_of::<f32>()) as u64));
    }
    elem.used = data.len()
}


// https://github.com/makepad/makepad.github.io/blob/3b4c0ba22cb02ba87d067c0f73b013c66e8bd6bd/render/src/cx_metal.rs#L551