fn new_box_with<T>(new_fn: impl FnOnce(*mut T) -> T) -> Box<T> {
    let b = Box::new(MaybeUninit::<T>::uninit());
    let p = Box::into_raw(b) as *mut T;
    unsafe { ptr::write(p, new_fn(p)) };
    unsafe { Box::from_raw(p) }
  }


// https://github.com/denosaurs/deno_desktop/blob/f4886bea938fad361f3a7efb6a22704b5fdafdce/core/inspector.rs#L719