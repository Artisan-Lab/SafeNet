fn next(&mut self) -> Option<&'cur mut T> {
    let cur = match self.cur.map(Node::as_mut) {
        Some(node) if self.idx < node.len => node,
        _ => return None,
    };

    let item = unsafe { cur.data.get_unchecked_mut(self.idx).assume_init_mut() };

    self.idx += 1;

    if self.idx == cur.len {
        self.idx = 0;
        self.cur = &mut cur.next;
    }

    Some(item)
}
// https://github.com/maciejhirsz/kobold/blob/6ffab778726486f29c3a207a2870c8f2bfc55037/crates/kobold/src/list/page_list.rs#L255