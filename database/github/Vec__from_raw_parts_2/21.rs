fn test_memory_allocate() {
    let size = 10;
    let mem = __quantum__rt__memory_allocate(size as u64);
    unsafe {
        for val in Vec::from_raw_parts(mem, size, size) {
            assert_eq!(val, 0);
        }
    }
    }
    /*
    https://github.com/qir-alliance/qir-runner/blob/e311bee03991c9b33b10b632e83d069870547cd5/stdlib/src/lib.rs#L101
    */