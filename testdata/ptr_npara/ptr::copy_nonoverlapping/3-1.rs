pub(crate) fn from_nsdata(data: id) -> Vec<u8> {
    unsafe {
        let len: NSUInteger = msg_send![data, length];
        let bytes: *const c_void = msg_send![data, bytes];
        let mut out: Vec<u8> = Vec::with_capacity(len as usize);
        std::ptr::copy_nonoverlapping(bytes as *const u8, out.as_mut_ptr(), len as usize);
        out.set_len(len as usize);
        out
    }
}
// https://github.com/linebender/druid/blob/e53a5ab72c40191b3f92edef9ebf4da07da254f3/druid-shell/src/backend/mac/util.rs#L60