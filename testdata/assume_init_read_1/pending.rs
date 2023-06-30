fn one_node() {
    let list = PageList::build(0..128, |n, p| p.put(n));

    let first = Node::as_mut(list.first.unwrap());

    unsafe {
        assert_eq!(first.data[0].assume_init_read(), 0);
        assert_eq!(first.data[127].assume_init_read(), 127);
    }
}