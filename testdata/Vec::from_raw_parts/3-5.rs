pub extern "C" fn cvec_drop(cvec: CVec) {
    let CVec { ptr, len, cap } = cvec;
    let data: Vec<u8> = unsafe { Vec::from_raw_parts(ptr.cast::<u8>(), len, cap) };
    drop(data); // Memory freed here
}

//https://github.com/nautechsystems/nautilus_trader/blob/00517f3d302ca060202f62a363d6351805e6c8dc/nautilus_core/core/src/cvec.rs#L93