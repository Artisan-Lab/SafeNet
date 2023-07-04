#![allow(unused)]

fn main() {
    unsafe {
        std::ptr::write(got_entry, addr);
    }
    let plt_entry = memory
        .code
        .allocate(std::mem::size_of::<[u8; 16]>(), EXECUTABLE_DATA_ALIGNMENT)
        .unwrap()
        .cast::<[u8; 16]>();
    libcall_plt_entries.insert(libcall, NonNull::new(plt_entry).unwrap());
    unsafe {
        Self::write_plt_entry_bytes(plt_entry, got_entry);
    }

}