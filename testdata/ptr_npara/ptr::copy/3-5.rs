fn write_str(&mut self, s: &str) -> core::fmt::Result {
    let data = s.as_bytes();

    unsafe {
        core::ptr::copy(
            data.as_ptr(),
            PANIC_INFO.as_mut_ptr() as *mut u8,
            core::cmp::min(data.len(), PANIC_INFO_SIZE));
    }

    Ok(())
}
// https://github.com/frida/frida-core/blob/db40941e8d9501c635a4328e6ca5eb61cfc2329a/src/barebone/helpers/bin/memory_scanner.rs#L40