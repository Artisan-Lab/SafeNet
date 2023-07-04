fn drop(&mut self) {
    let mut cursor = self.head;
    while !cursor.is_null() {
        unsafe {
            let node = Box::from_raw(cursor);
            cursor = node.prev;
            drop(node);
        }
    }
}

// https://github.com/ciusji/sled/blob/5fdddf0d0af8144466050f396180ee259f2dcf09/src/dll.rs#L75