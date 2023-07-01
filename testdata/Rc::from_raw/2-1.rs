/*
https://github.com/rustwasm/wasm-bindgen/blob/165ad00ccff4bc67705afdacecf894cb8250ee2e/crates/futures/src/task/singlethread.rs#L64
*/

unsafe fn raw_wake(ptr: *const ()) {
    let ptr = Rc::from_raw(ptr as *const Task);
    Task::wake_by_ref(&ptr);
}
