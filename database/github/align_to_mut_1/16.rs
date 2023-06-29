fn align_from_bytes() {
    let values = &mut [42u64, 69];
    let bytes = unsafe { values.align_to_mut().1 };

    let size = ::core::mem::size_of::<u64>();

    assert_eq!(
        AlignedSpace::<u64>::align_from_bytes_mut(bytes)
            .unwrap()
            .bytes_len(),
        size * 2
    );

    let ptr = bytes[9..11].as_ptr();
}

// https://github.com/RustAudio/rust-lv2/blob/44aac000be49216ddd00f46aa7e8a05bde4a09f7/atom/src/space/aligned.rs#L522