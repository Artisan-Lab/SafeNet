fn drop(&mut self) {
    for (iov, is_owned) in self.iov.iter().zip(self.is_owned.iter()) {
        if *is_owned {
            // Free the owned string.  Safe because we recorded ownership,
            // and Nmount does not implement Clone.
            unsafe {
                drop(CString::from_raw(iov.iov_base as *mut c_char));
            }
        }
    }
}