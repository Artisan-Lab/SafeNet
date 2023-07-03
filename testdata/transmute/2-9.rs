fn test_str_to_bytes_horribly_unsafe() {
        let bytes = unsafe { std::mem::transmute::<&str, &[u8]>("Going off the menu") };
        assert_eq!(
            bytes,
            &[
                71, 111, 105, 110, 103, 32, 111, 102, 102, 32, 116, 104, 101, 32, 109, 101, 110,
                117
            ]
        );
    }
