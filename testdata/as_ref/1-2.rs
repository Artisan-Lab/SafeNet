fn get_sender(sender: *const c_void) -> Option<Sender<Result<BufView, Error>>> {
    // SAFETY: We know this is a valid v8::External
    unsafe {
      (sender as *const SenderCell)
        .as_ref()
        .and_then(|r| r.borrow_mut().as_ref().cloned())
    }
  }
// https://github.com/paramk90/deno/blob/3a2d284c96a7fe0a4a0d0430df79b79b8a4a4422/ext/web/stream_resource.rs#L194