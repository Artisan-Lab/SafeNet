pub fn get_schema(&mut self, out: *mut FFI_ArrowSchema) -> i32 {
    let private_data = self.get_private_data();
    let reader = &private_data.batch_reader;

    let schema = FFI_ArrowSchema::try_from(reader.schema().as_ref());

    match schema {
        Ok(schema) => {
            unsafe { std::ptr::copy(addr_of!(schema), out, 1) };
            std::mem::forget(schema);
            0
        }
        Err(ref err) => {
            private_data.last_error = Some(
                CString::new(err.to_string())
                    .expect("Error string has a null byte in it."),
            );
            get_error_code(err)
        }
    }
}
// https://github.com/apache/arrow-rs/blob/2c4bc5449fc4432ecb2f9963994ac8997b64b52e/arrow/src/ffi_stream.rs#L176