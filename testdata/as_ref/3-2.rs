use std::ptr::NonNull;
fn size(&self) -> u32 {
    unsafe {
        let ptr = self.get_vm_table_definition();
        let td = ptr.as_ref();
        td.current_elements
    }
}