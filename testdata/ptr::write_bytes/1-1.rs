pub fn new(width: usize, height: usize, stride: usize, onscreen_ptr: *mut u32) -> Display {
    let size = stride * height;
    let onscreen = unsafe {
        ptr::write_bytes(onscreen_ptr, 0, size);
        slice::from_raw_parts_mut(onscreen_ptr, size)
    };
    Display {
        width,
        height,
        stride,
        onscreen,
        offscreen: None,
    }
}
// https://github.com/redox-os/kernel/blob/7269f9c6f19956017ce36484937d358b8fa1299c/src/devices/graphical_debug/display.rs#L19