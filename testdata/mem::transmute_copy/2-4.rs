pub fn get_caption() -> (String, String) {
    let mut title_buf = ptr::null_mut();
    let mut icon_buf = ptr::null_mut();
    let mut title = String::new();
    let mut icon = String::new();

    unsafe {
        ll::SDL_WM_GetCaption(&mut title_buf, &mut icon_buf);

        if !title_buf.is_null() {
            let slice = CStr::from_ptr(mem::transmute_copy(&mut &title_buf)).to_bytes();
            title = str::from_utf8(slice).unwrap().to_string();
        }

        if !icon_buf.is_null() {
            let slice = CStr::from_ptr(mem::transmute_copy(&mut &icon_buf)).to_bytes();
            icon = str::from_utf8(slice).unwrap().to_string();
        }

        (title, icon)
    }
}

// https://github.com/brson/rust-sdl/blob/78bd0e749ca99d5c4c1a63a1759d494f7a4f94c7/src/sdl/wm.rs#L57