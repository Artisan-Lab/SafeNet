fn clone_shared_waker(self_ptr: *const ()) -> RawWaker {
    let self_weak = unsafe { Weak::from_raw(self_ptr as *const Self) };
    let ptr1 = self_weak.clone().into_raw();
    let ptr2 = self_weak.into_raw();
    assert!(ptr1 == ptr2);
    RawWaker::new(self_ptr, &Self::SHARED_WAKER_VTABLE)
  }

  // https://github.com/easyops-cn/deno/blob/5b14a9c3409de06c1d643a3831822db454924fc3/extensions/net/ops_tls.rs#L593