/*
https://github.com/servo/servo/blob/041d95e0f4ef08dddab22e0276204240682cdf79/components/script/dom/xmlhttprequest.rs#L1466
*/
fn json_response(&self, cx: JSContext) -> JSVal {
        // Step 1
        let response_json = self.response_json.get();
        if !response_json.is_null_or_undefined() {
            return response_json;
        }
        // Step 2
        let bytes = self.response.borrow();
        // Step 3
        if bytes.len() == 0 {
            return NullValue();
        }
        // Step 4
        fn decode_to_utf16_with_bom_removal(bytes: &[u8], encoding: &'static Encoding) -> Vec<u16> {
            let mut decoder = encoding.new_decoder_with_bom_removal();
            let capacity = decoder
                .max_utf16_buffer_length(bytes.len())
                .expect("Overflow");
            let mut utf16 = Vec::with_capacity(capacity);
            let extra = unsafe { slice::from_raw_parts_mut(utf16.as_mut_ptr(), capacity) };
            let last = true;
            let (_, read, written, _) = decoder.decode_to_utf16(bytes, extra, last);
            assert_eq!(read, bytes.len());
            unsafe { utf16.set_len(written) }
            utf16
        }
        // https://xhr.spec.whatwg.org/#json-response refers to
        // https://infra.spec.whatwg.org/#parse-json-from-bytes which refers to
        // https://encoding.spec.whatwg.org/#utf-8-decode which means
        // that the encoding is always UTF-8 and the UTF-8 BOM is removed,
        // if present, but UTF-16BE/LE BOM must not be honored.
        let json_text = decode_to_utf16_with_bom_removal(&bytes, UTF_8);
        // Step 5
        rooted!(in(*cx) let mut rval = UndefinedValue());
        unsafe {
            if !JS_ParseJSON(
                *cx,
                json_text.as_ptr(),
                json_text.len() as u32,
                rval.handle_mut(),
            ) {
                JS_ClearPendingException(*cx);
                return NullValue();
            }
        }
        // Step 6
        self.response_json.set(rval.get());
        self.response_json.get()
    }