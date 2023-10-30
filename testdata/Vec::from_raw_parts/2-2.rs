fn drop(&mut self) {
    unsafe {
        if self.went_off_stack() {
            // free the heap
            let (ptr, len) = self.store.heap();
            // let vec's destructor do the work
            mem::drop(Vec::from_raw_parts(ptr, len, self.cap));
        } else {
            // on stack? get self as a slice and destruct it
            ptr::drop_in_place(&mut self[..]);
        }
    }
}

// https://github.com/skytable/skytable/blob/6736285df5f94ae6e2144ee84cbfd128e1a63d8b/server/src/corestore/iarray.rs#L543