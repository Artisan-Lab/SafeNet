pub extern "C" fn on_file_dropped(
    path: *mut u8,
    path_len: usize,
    bytes: *mut u8,
    bytes_len: usize,
) {
    tl_display::with(|display| {
        let path = PathBuf::from(unsafe { String::from_raw_parts(path, path_len, path_len) });
        let bytes = unsafe { Vec::from_raw_parts(bytes, bytes_len, bytes_len) };

        display.dropped_files.paths.push(path);
        display.dropped_files.bytes.push(bytes);
    });
}

// https://github.com/not-fl3/miniquad/blob/4b5fe80d8f26ea3c1d0613c143a117c1b2dde430/src/native/wasm.rs#L412