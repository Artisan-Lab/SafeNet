fn into_string(self) -> Rc<StringObj> {
    unsafe { Rc::from_raw(self.string) }
}