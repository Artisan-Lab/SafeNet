pub fn get_dictionary(&self) -> Option<Vec<u8>> {
    let mut dict = Vec::with_capacity(32_768);
    let mut dict_length = 0;
    unsafe {
        let r = deflateGetDictionary(self.strm, dict.as_mut_ptr(), &mut dict_length);
        if r == Z_OK {
            dict.truncate(dict_length);
            //set_len()
            Some(dict)
        } else {
            None
        }
    }
}
