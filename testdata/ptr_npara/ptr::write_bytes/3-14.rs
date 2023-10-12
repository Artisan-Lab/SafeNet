unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
    // use crate::alloc::{GlobalAlloc, Layout, System};
    let addr = abi::malloc(layout.size(), layout.align());

    if !addr.is_null() {
        ptr::write_bytes(addr, 0x00, layout.size());
    }

    addr
}
// https://github.com/glaubitz/rust/blob/1f5768bc67ecb025342770e14e03699699965706/library/std/src/sys/hermit/alloc.rs#L16