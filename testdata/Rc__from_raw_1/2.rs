unsafe fn into_raw_waker(this: Rc<Self>) -> RawWaker {
    unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
        let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
        Task::into_raw_waker((*ptr).clone())
    }

    unsafe fn raw_wake(ptr: *const ()) {
        let ptr = Rc::from_raw(ptr as *const Task);
        Task::wake_by_ref(&ptr);
    }

    unsafe fn raw_wake_by_ref(ptr: *const ()) {
        let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
        Task::wake_by_ref(&ptr);
    }

    unsafe fn raw_drop(ptr: *const ()) {
        drop(Rc::from_raw(ptr as *const Task));
    }

    const VTABLE: RawWakerVTable =
        RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

    RawWaker::new(Rc::into_raw(this) as *const (), &VTABLE)
}
/*
https://github.com/rustwasm/wasm-bindgen/blob/165ad00ccff4bc67705afdacecf894cb8250ee2e/crates/futures/src/task/singlethread.rs#L60
*/