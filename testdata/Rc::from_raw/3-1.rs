/*
  From: https://github.com/theseus-os/Theseus/blob/4a7d29a5b55065cfd7ca9bbb1edce795f545ff43/ports/theseus_std/src/os_str_imp.rs#L194
*/

pub fn into_rc(&self) -> Rc<Slice> {
    let rc: Rc<str> = Rc::from(&self.inner);
    unsafe { Rc::from_raw(Rc::into_raw(rc) as *const Slice) }
}
