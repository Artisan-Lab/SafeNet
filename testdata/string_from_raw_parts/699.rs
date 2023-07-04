/*
From: https://github.com/diorahman/neon/blob/05d072448e60bba83fe7c6bcb5c5a3099e35e367/src/internal/value.rs#L266
*/

pub fn data(self) -> std::string::String {
    unsafe {
        // FIXME: use StringBytes::StorageSize instead?
        // FIXME: audit all these isize -> usize casts
        let capacity = NeonSys_String_Utf8Length(self.to_raw());
        let mut buffer: Vec<u8> = Vec::with_capacity(capacity as usize);
        let p = buffer.as_mut_ptr();
        mem::forget(buffer);
        let len = NeonSys_String_Data(p, capacity, self.to_raw());
        std::string::String::from_raw_parts(p, len as usize, capacity as usize)
    }
}