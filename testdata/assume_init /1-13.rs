fn encode_value(mut self, encoder: &mut Encoder, context: Context) {
    let mut panic_error = None;
    let mut moves = 0;
    unsafe {
        // In order to move elements out of an array we need to replace the
        // value with uninitialized memory.
        for elem in self.iter_mut() {
            let owned_elem = mem::replace(elem, mem::MaybeUninit::uninit().assume_init());
            // We need to handle if an unwinding panic happens to prevent use of
            // uninitialized memory...
            let next_context = context.clone();
            // We assert everything going into this closure is unwind safe. If anything
            // is added, PLEASE make sure it is also unwind safe...
            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                owned_elem.encode(encoder, next_context);
            }));
            if let Err(err) = result {
                panic_error = Some(err);
                break;
            }
            moves += 1;
        }
        if let Some(err) = panic_error {
            for i in moves..self.len() {
                ptr::drop_in_place(&mut self[i] as *mut T);
            }
            // Forget the array to prevent a drop
            mem::forget(self);
            // Continue unwinding
            panic::resume_unwind(err);
        }
        // We cannot risk drop() getting run on the array values, so we just
        // forget self.
        mem::forget(self);
    }