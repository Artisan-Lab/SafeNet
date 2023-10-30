fn drop(&mut self) {
    unsafe {
        if !self.errMsg.is_null() {
            CString::from_raw(self.errMsg as *mut c_char);
        }
        if self.len > 0 {
            Vec::from_raw_parts(self.data as *mut u8, self.len as usize, self.len as usize);
        }
    }
}


// https://github.com/alldatacenter/alldata/blob/2fc8f53f7f49f27354d65b02635af014f1b0aea1/kg/GraphScope/interactive_engine/executor/assembly/groot/src/store/jna_response.rs#L84