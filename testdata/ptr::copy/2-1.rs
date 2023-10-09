unsafe fn slice_out(&mut self, child: NodePtr<E, I, IE, LE>) -> Node<E, I, IE, LE> {
    if self.children[1].is_none() {
        // short circuit.

        // If we're in this situation, children[0] must be Some(child).
        debug_assert_eq!(self.find_child(child).unwrap(), 0);

        self.children[0].take().unwrap()
    } else {
        let idx = self.find_child(child).unwrap();
        let num_children = self.count_children();

        let removed = self.children[idx].take().unwrap();

        let count = num_children - idx - 1;
        if count > 0 {
            ptr::copy(
                &self.children[idx + 1],
                &mut self.children[idx],
                count
            );

            self.metrics.copy_within(idx + 1..num_children, idx);
        }

        // This pointer has been moved. We need to set its entry to None without dropping it.
        std::mem::forget(self.children[num_children - 1].take());

        removed
    }
}
// https://github.com/josephg/diamond-types/blob/1caffd46485f3db28a2fa8a77d4c8d7d004e12ae/crates/content-tree/src/mutations.rs#L1031

// fn slice_out(&mut self, child: NodePtr<E, I, IE, LE>) -> Node<E, I, IE, LE> {
//     if self.children[1].is_none() {
//         debug_assert_eq!(self.find_child(child).unwrap(), 0);

//         unsafe {
//             self.children[0].take().unwrap()
//         }
//     } else {
//         let idx = self.find_child(child).unwrap();
//         let num_children = self.count_children();

//         let removed = unsafe { self.children[idx].take().unwrap() };

//         let count = num_children - idx - 1;
//         if count > 0 {
//             unsafe {
//                 ptr::copy(
//                     &self.children[idx + 1],
//                     &mut self.children[idx],
//                     count
//                 );

//                 self.metrics.copy_within(idx + 1..num_children, idx);
//             }
//         }
//         unsafe {
//             std::mem::forget(self.children[num_children - 1].take());
//         }

//         removed
//     }
// }
