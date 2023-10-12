impl<T: RawDeviceId, U, const N: usize> IdArray<T, U, N> {
    /// Creates a new instance of the array.
    ///
    /// The contents are derived from the given identifiers and context information.
    pub const fn new(ids: [T; N], infos: [Option<U>; N]) -> Self
        where
            T: ~ const RawDeviceId + Copy,
            T::RawType: Copy + Clone,
    {
        let mut array = Self {
            ids: IdArrayIds {
                ids: [T::ZERO; N],
                sentinel: T::ZERO,
            },
            id_infos: infos,
        };
        let mut i = 0usize;
        while i < N {
            // SAFETY: Both pointers are within `array` (or one byte beyond), consequently they are
            // derived from the same allocated object. We are using a `u8` pointer, whose size 1,
            // so the pointers are necessarily 1-byte aligned.
            let offset = unsafe {
                (&array.id_infos[i] as *const _ as *const u8)
                    .offset_from(&array.ids.ids[i] as *const _ as _)
            };
            array.ids.ids[i] = ids[i].to_rawid(offset);
            i += 1;
        }
        array
    }
}
//https://github.com/AsahiLinux/linux/blob/b5c05cbffb0488c7618106926d522cc3b43d93d5/rust/kernel/driver.rs#L183
