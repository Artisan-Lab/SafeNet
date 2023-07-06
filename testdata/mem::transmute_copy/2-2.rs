pub fn from_array(array: [bool; LANES]) -> Self {
    // SAFETY: Rust's bool has a layout of 1 byte (u8) with a value of
    //     true:    0b_0000_0001
    //     false:   0b_0000_0000
    // Thus, an array of bools is also a valid array of bytes: [u8; N]
    // This would be hypothetically valid as an "in-place" transmute,
    // but these are "dependently-sized" types, so copy elision it is!
    unsafe {
        let bytes: [u8; LANES] = mem::transmute_copy(&array);
        let bools: Simd<i8, LANES> =
            intrinsics::simd_ne(Simd::from_array(bytes), Simd::splat(0u8));
        Mask::from_int_unchecked(intrinsics::simd_cast(bools))
    }
}

// https://github.com/programmerjake/portable-simd/blob/691c8b29e619d0e7f19b519bb6a9768345615200/crates/core_simd/src/masks.rs#L131