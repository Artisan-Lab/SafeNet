/*
https://github.com/hayd/deno/blob/c3e48cba184f2f8aaf3d30196b674c8a7dd8449b/serde_v8/magic/bytestring.rs#L60
*/

fn from_v8(
    scope: &mut v8::HandleScope,
    value: v8::Local<v8::Value>,
  ) -> Result<Self, crate::Error> {
    let v8str = v8::Local::<v8::String>::try_from(value)
      .map_err(|_| Error::ExpectedString)?;
    if !v8str.contains_only_onebyte() {
      return Err(Error::ExpectedLatin1);
    }
    let len = v8str.length();
    let mut buffer = SmallVec::with_capacity(len);
    #[allow(clippy::uninit_vec)]
    // SAFETY: we set length == capacity (see previous line),
    // before immediately writing into that buffer and sanity check with an assert
    unsafe {
      buffer.set_len(len);
      let written = v8str.write_one_byte(
        scope,
        &mut buffer,
        0,
        v8::WriteOptions::NO_NULL_TERMINATION,
      );
      assert!(written == len);
    }
    Ok(Self(buffer))
  }