fn drop(&mut self) {
    unsafe {
        let _guard = DropGuard::new();
        while let Some(node) = self.boxes_start {
            let node = Box::from_raw(node.as_ptr());
            self.boxes_start = node.header.next;
        }

    }
}