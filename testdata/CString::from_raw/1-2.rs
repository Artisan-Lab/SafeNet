pub extern "C" fn free_cstr(s: *mut c_char) {
    if s.is_null() {
        println!("passed a null object");
        return;
    }

    unsafe {
        let _ = CString::from_raw(s);
    }
}

//https://github.com/zerotier/ZeroTierOne/blob/0962af5e724a40a3856262dc134a541d59dfc816/zeroidc/src/ext.rs#L210