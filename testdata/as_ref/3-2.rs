fn size(&self) -> u32 {
    use std::ptr::NonNull;
    unsafe {
        let ptr = self.get_vm_table_definition();
        let td = ptr.as_ref();
        td.current_elements
    }
}