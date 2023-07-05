/*
https://github.com/tyrchen/deno/blob/9c5928b5aa3716f7441694da24982cecacb7a061/serde_v8/magic/u16string.rs#L33
*/

fn from_v8(
    scope: &mut v8::HandleScope,
    value: v8::Local<v8::Value>,
  ) -> Result<Self, crate::Error> {
    let v8str = v8::Local::<v8::String>::try_from(value)
      .map_err(|_| Error::ExpectedString)?;
    let len = v8str.length();
    let mut buffer = Vec::with_capacity(len);
    // SAFETY: we set length == capacity (see previous line),
    // before immediately writing into that buffer and sanity check with an assert
    #[allow(clippy::uninit_vec)]
    unsafe {
      buffer.set_len(len);
      let written = v8str.write(
        scope,
        &mut buffer,
        0,
        v8::WriteOptions::NO_NULL_TERMINATION,
      );
      assert!(written == len);
    }
    Ok(buffer.into())
  }