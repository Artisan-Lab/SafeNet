/*
https://github.com/emacs-ng/deno/blob/ffffa2f7c44bd26aec5ae1957e0534487d099f48/serde_v8/de.rs#L739
*/
fn to_utf8_fast(
  s: v8::Local<v8::String>,
  scope: &mut v8::HandleScope,
) -> Option<String> {
  // Over-allocate by 20% to avoid checking string twice
  let len = s.length();
  let capacity = (len as f64 * 1.2) as usize;
  let mut buf = Vec::with_capacity(capacity);
  let mut nchars = 0;
  let data = buf.as_mut_ptr();
  let length = s.write_utf8(
    scope,
    // SAFETY: we're essentially providing the raw internal slice/buffer owned by the Vec
    // which fulfills all of from_raw_parts_mut's safety requirements besides "initialization"
    // and since we're operating on a [u8] not [T] we can safely assume the slice's values
    // are sufficiently "initialized" for writes
    unsafe { std::slice::from_raw_parts_mut(data, capacity) },
    Some(&mut nchars),
    v8::WriteOptions::NO_NULL_TERMINATION
      | v8::WriteOptions::REPLACE_INVALID_UTF8,
  );
  if nchars < len {
    return None;
  }
  // SAFETY: write_utf8 guarantees `length` bytes are initialized & valid utf8
  unsafe {
    buf.set_len(length);
    Some(String::from_utf8_unchecked(buf))
  }
}
