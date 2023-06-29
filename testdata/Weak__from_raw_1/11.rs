fn with_state<F>(this: &Object, callback: F)
where
    F: FnOnce(Rc<PlatformWindow>),
{
    let state = unsafe {
        let state_ptr = {
            let state_ptr: *mut c_void = *this.get_ivar("imState");
            state_ptr as *const PlatformWindow
        };
        ManuallyDrop::new(Weak::from_raw(state_ptr))
    };
    let upgraded = state.upgrade();
    if let Some(upgraded) = upgraded {
        callback(upgraded);
    }
}
/*
https://github.com/OpenRAS/nativeshell/blob/ec64057cd8bd0a5e3949130f5fa14bb6afefa782/nativeshell/src/shell/platform/macos/window.rs#L1035
*/