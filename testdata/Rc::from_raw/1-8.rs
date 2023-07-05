unsafe fn v8slice_clone(
    data: &rawbytes::AtomicPtr<()>,
    ptr: *const u8,
    len: usize,
  ) -> bytes::Bytes {
    let rc = Rc::from_raw(*data as *const V8Slice);
    let (_, _, data) = rc.clone().rc_into_byte_parts();
    std::mem::forget(rc);
    // NOTE: `bytes::Bytes` does bounds checking so we trust its ptr, len inputs
    // and must use them to allow cloning Bytes it has sliced
    rawbytes::RawBytes::new_raw(ptr, len, data.cast(), &V8SLICE_VTABLE)
  }