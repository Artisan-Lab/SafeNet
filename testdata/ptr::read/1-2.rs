pub fn read(&self, index: usize) -> u8 {
    unsafe { ptr::read_volatile((&self.buffer[index]) as *const u8) }
}
// https://github.com/otcshare/chromium-src/blob/3b920d87437d9293f654de1f22d3ea341e7a8b55/mojo/public/rust/system/shared_buffer.rs#L69