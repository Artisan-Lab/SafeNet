unsafe extern "system" fn win_proc_dispatch(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // if msg == WM_CREATE {
    //     let create_struct = &*(lparam as *const CREATESTRUCTW);
    //     let wndproc_ptr = create_struct.lpCreateParams;
    //     SetWindowLongPtrW(hwnd, GWLP_USERDATA, wndproc_ptr as WindowLongPtr);
    // }
    let window_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const WindowState;
    let result = {
        if window_ptr.is_null() {
            None
        } else {
            (*window_ptr).wndproc.window_proc(hwnd, msg, wparam, lparam)
        }
    };

    if msg == WM_NCDESTROY && !window_ptr.is_null() {
        (*window_ptr).wndproc.cleanup(hwnd);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        mem::drop(Rc::from_raw(window_ptr));
    }

    match result {
        Some(lresult) => lresult,
        None => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
