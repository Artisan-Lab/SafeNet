/*
  From: https://github.com/rustwasm/wasm-bindgen/blob/165ad00ccff4bc67705afdacecf894cb8250ee2e/crates/futures/src/task/singlethread.rs#L69
*/

unsafe fn raw_wake_by_ref(ptr: *const ()) {
    let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
    Task::wake_by_ref(&ptr);
}
