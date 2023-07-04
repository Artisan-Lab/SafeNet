fn convert_from_js( data : *mut c_char ) -> String {
    unsafe {
        let s = CString::from_raw(data);
        let s = s.into_string().unwrap();    
        let result = s.clone();
        mem::forget(s);
        result
    }    
}
