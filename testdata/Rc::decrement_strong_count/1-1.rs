/*
    From: https://github.com/brownben/bang2/blob/368f117463057c0bfc82776f7ccc1004b7ce3e3d/interpreter/src/value/bit64.rs#L73
*/ 

fn drop(&mut self) {
    if self.is_object() {
      let pointer = self.0.map_addr(|ptr| ptr & FROM_STORED);
      unsafe { Rc::decrement_strong_count(pointer) };
    } else if self.is_allocated() {
      let pointer = self.0.map_addr(|ptr| ptr & FROM_STORED_ADDRESS);
      let pointer = pointer.cast::<RefCell<Self>>();
      unsafe { Rc::decrement_strong_count(pointer) };
    }
}