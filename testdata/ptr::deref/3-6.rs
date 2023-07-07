fn test_null_terminated_array() {
    let owned_strs = &[CString::new("foo").unwrap(), CString::new("bar").unwrap()];
    let strs = owned_strs.iter().map(|s| s.as_c_str()).collect::<Vec<_>>();
    let arr = NullTerminatedArray::new(&strs);
    let ptr = arr.get();
    unsafe {
        assert_eq!(CStr::from_ptr(*ptr).to_str().unwrap(), "foo");
        assert_eq!(CStr::from_ptr(*ptr.offset(1)).to_str().unwrap(), "bar");
        assert_eq!(*ptr.offset(2), ptr::null());
    }
}

//https://github.com/doytsujin/fish-shell/blob/93dc8485dd05efa873f9822321a4b9aa977c7176/fish-rust/src/null_terminated_array.rs#L117