pub unsafe fn alloc_zeroed(count: u32) -> *mut u8 {
    let raw = alloc(count);
    if !raw.is_null() {
        ptr::write_bytes(raw, 0, count as usize * SE_PAGE_SIZE);
    }
    raw
}
// https://github.com/apache/incubator-teaclave-sgx-sdk/blob/f1776a7cec1caab2959813f87bb4924805b92011/sgx_alloc/src/rsrvmem.rs#L175