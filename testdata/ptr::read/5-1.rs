fn next(&mut self) -> Option<T> {
    unsafe {
        while self.idx < self.old_len {
            let i = self.idx;
            let v = slice::from_raw_parts_mut(self.vec.as_mut_ptr(), self.old_len);
            let drained = (self.pred)(&mut v[i]);
            // Update the index *after* the predicate is called. If the index
            // is updated prior and the predicate panics, the element at this
            // index would be leaked.
            self.idx += 1;
            if drained {
                self.del += 1;
                return Some(ptr::read(&v[i]));
            } else if self.del > 0 {
                let del = self.del;
                let src: *const T = &v[i];
                let dst: *mut T = &mut v[i - del];
                ptr::copy_nonoverlapping(src, dst, 1);
            }
        }
        None
    }
}

// https://github.com/rust-lang/rust/blob/b7bc6f88ac771e1197ac9e06bf20586eeb5d0bdf/library/alloc/src/vec/extract_if.rs#L73


// fn next(&mut self) -> Option<T> {
//     while self.idx < self.old_len {
//         let i = self.idx;
//         let v = slice::from_raw_parts_mut(self.vec.as_mut_ptr(), self.old_len);
//         let drained = (self.pred)(&mut v[i]);
//         self.idx += 1;
//         if drained {
//             self.del += 1;
//             return Some(unsafe { ptr::read(&v[i]) });
//         } else if self.del > 0 {
//             let del = self.del;
//             let src: *const T = &v[i];
//             let dst: *mut T = &mut v[i - del];
//             unsafe {
//                 ptr::copy_nonoverlapping(src, dst, 1);
//             }
//         }
//     }
//     None
// }
