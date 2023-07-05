pub fn read(&self, index: usize) -> u8 {
    unsafe { ptr::read_volatile((&self.buffer[index]) as *const u8) }
}
// https://github.com/nwjs/chromium.src/blob/21945e5c163533b01a44214b663a9bf615c31ab5/mojo/public/rust/system/shared_buffer.rs#L69