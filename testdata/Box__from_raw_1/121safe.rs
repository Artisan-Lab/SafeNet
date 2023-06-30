fn drop(&mut self) {
    let _guard = DropGuard::new();
    let mut head = self.boxes_start.take();
    while let Some(node) = head {
        head = node.header.next;
    }
}