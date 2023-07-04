/*
From: https://github.com/cedric-h/slither/blob/7da78d353e16e37dca9994756d6142440aea3934/rust-gc/gc/src/gc.rs#L32
*/
/*
struct GcState {
    bytes_allocated: usize,
    threshold: usize,
    boxes_start: Option<NonNull<GcBox<Trace>>>,
}

impl Drop for GcState {
*/
    fn drop(&mut self) {
        unsafe {
            {
                let mut p = &self.boxes_start;
                while let Some(node) = *p {
                    Finalize::finalize(&(*node.as_ptr()).data);
                    p = &(*node.as_ptr()).header.next;
                }
            }

            let _guard = DropGuard::new();
            while let Some(node) = self.boxes_start {
                let node = Box::from_raw(node.as_ptr());
                self.boxes_start = node.header.next;
            }
        }
    }
//}