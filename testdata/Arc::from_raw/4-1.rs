/*
https://github.com/dinfuehr/dora/blob/d72ac4a1622e6c8038ee867bda61aea0cb3be849/dora-runtime/src/vm/code.rs#L67
*/

pub fn drop_native_code_object(&mut self) -> Arc<Code> {
    let native_code = unsafe { Arc::from_raw(self.native_code_object.to_ptr::<Code>()) };
    self.native_code_object = Address::null();
    native_code
}