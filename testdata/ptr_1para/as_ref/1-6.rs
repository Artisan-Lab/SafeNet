// pub type uv_async_t = *mut uv_async;
pub unsafe extern "C" fn uv_async_send(async_: *mut uv_async) -> c_int {
    let sender = (*async_).sender.as_ref().unwrap();
    let fut = Box::new(move || {
      ((*async_).callback)(async_);
    });
  
    match sender.unbounded_send(fut) {
      Ok(_) => 0,
      Err(_) => 1,
    }
  }

// https://github.com/milahu/deno/blob/fa22956a8616c34482b10bb3ae1aed76ad017c3e/cli/napi/mod.rs#L78