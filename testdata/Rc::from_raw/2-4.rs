/*
  From: https://github.com/rustwasm/wasm-bindgen/blob/165ad00ccff4bc67705afdacecf894cb8250ee2e/crates/futures/src/task/singlethread.rs#L74
*/

unsafe fn raw_drop(ptr: *const ()) {
    drop(Rc::from_raw(ptr as *const Task));
}
