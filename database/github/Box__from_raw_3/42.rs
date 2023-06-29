extern "C" fn post_task_handler(callback: *mut c_void) {
    let mut closure = unsafe { Box::from_raw(callback as *mut Box<dyn FnMut()>) };

    closure()
}
/*
https://github.com/fathyb/carbonyl/blob/ab80a276b1bd1c2c8dcefc8f248415dfc61dc2bf/src/browser/bridge.rs#L305
*/