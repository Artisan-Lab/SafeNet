fn path_to_bytes(p: &Path) -> Vec<u8> {
    let mut source = p.as_os_str().encode_wide().collect::<Vec<u16>>();

    let ptr = source.as_mut_ptr() as *mut u8;
    let len = source.len() * std::mem::size_of::<u16>();
    let capacity = source.capacity() * std::mem::size_of::<u16>();
    unsafe {
        std::mem::forget(source);
        Vec::from_raw_parts(ptr, len, capacity)
    }
}

// https://github.com/indygreg/PyOxidizer/blob/b78b0cb75f4317c45408bbc9a569c062c482c679/python-packed-resources/src/writer.rs#L50