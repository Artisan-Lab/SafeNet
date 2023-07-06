pub fn to_array(self) -> [bool; LANES] {
    // This follows mostly the same logic as from_array.
    // SAFETY: Rust's bool has a layout of 1 byte (u8) with a value of
    //     true:    0b_0000_0001
    //     false:   0b_0000_0000
    // Thus, an array of bools is also a valid array of bytes: [u8; N]
    // Since our masks are equal to integers where all bits are set,
    // we can simply convert them to i8s, and then bitand them by the
    // bitpattern for Rust's "true" bool.
    // This would be hypothetically valid as an "in-place" transmute,
    // but these are "dependently-sized" types, so copy elision it is!
    unsafe {
        let mut bytes: Simd<i8, LANES> = intrinsics::simd_cast(self.to_int());
        bytes &= Simd::splat(1i8);
        mem::transmute_copy(&bytes)
    }
}

// https://github.com/programmerjake/portable-simd/blob/691c8b29e619d0e7f19b519bb6a9768345615200/crates/core_simd/src/masks.rs#L131