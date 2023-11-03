impl FrameObject for v3_11_0::_PyInterpreterFrame {
    type CodeObject = v3_11_0::PyCodeObject;
    fn code(&self) -> *mut Self::CodeObject {
        self.f_code
    }
    fn lasti(&self) -> i32 {
        // this returns the delta from the co_code, but we need to adjust for the
        // offset from co_code.co_code_adaptive. This is slightly easier to do in the
        // get_line_number code, so will adjust there
        let co_code = self.f_code as *const _ as *const u8;
        unsafe { (self.prev_instr as *const u8).offset_from(co_code) as i32 }
    }
    fn back(&self) -> *mut Self {
        self.previous
    }
}
//https://github.com/benfred/py-spy/blob/fea1c42a45701968f76a8babbd903d92a1092117/src/python_interpreters.rs#L356