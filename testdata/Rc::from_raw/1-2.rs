/*
  From: https://github.com/rustwasm/wasm-bindgen/blob/165ad00ccff4bc67705afdacecf894cb8250ee2e/crates/futures/src/task/singlethread.rs#L59
*/

unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
    let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
    Task::into_raw_waker((*ptr).clone())
}
