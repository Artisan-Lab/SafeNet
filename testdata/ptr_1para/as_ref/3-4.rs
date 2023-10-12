fn size(&self) -> Pages {
    use std::ptr::NonNull;
    unsafe {
        let md_ptr = self.get_vm_memory_definition();
        let md = md_ptr.as_ref();
        Bytes::from(md.current_length).try_into().unwrap()
    }
}