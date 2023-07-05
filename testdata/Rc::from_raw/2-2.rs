fn drop(&mut self) {
    unsafe {
        let idx = (*self.js.get()).idx;
        if idx != !0 {
            super::__wbindgen_cb_drop(idx);
        }
        drop(Rc::from_raw(self.inner));
    }
}