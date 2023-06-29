use std::ptr::NonNull;
fn size(&self) -> Pages {
    unsafe {
        let md_ptr = self.get_vm_memory_definition();
        let md = md_ptr.as_ref();
        Bytes::from(md.current_length).try_into().unwrap()
    }
}