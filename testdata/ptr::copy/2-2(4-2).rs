pub fn reserve_edges_dense(&mut self, degree_to_add: &Vec<i64>) {
    let vnum = self.vertex_num().index();
    assert_eq!(degree_to_add.len(), vnum);
    let mut new_buf_size: usize = 0;
    for i in 0..vnum {
        if degree_to_add[i] == 0 {
            continue;
        }
        let requirement = self.adj_lists[i].degree() + degree_to_add[i];
        if requirement > self.adj_lists[i].capacity() {
            self.remove_node(I::new(i));
            // requirement += (requirement + 1) / 2;
            new_buf_size += requirement as usize;
            self.adj_lists[i].set_capacity(-requirement);
        }
    }
    if new_buf_size != 0 {
        let layout = Layout::array::<Nbr<I>>(new_buf_size).unwrap();
        let new_buf = unsafe { alloc::alloc(layout) };
        let mut p = <I as IndexType>::max();
        let mut begin = new_buf as *mut Nbr<I>;
        for i in 0..vnum {
            let mut cap = self.adj_lists[i].capacity();
            if cap < 0 {
                cap = -cap;
                self.adj_lists[i].set_capacity(cap);
                self.prev[i] = p;
                if p != <I as IndexType>::max() {
                    self.next[p.index()] = I::new(i);
                }
                p = I::new(i);
                let old_degree = self.adj_lists[i].degree();
                if old_degree > 0 {
                    unsafe { ptr::copy(self.adj_lists[i].data(), begin, old_degree as usize) };
                }
                self.adj_lists[i].set(begin, old_degree, cap);
                begin = unsafe { begin.add(cap as usize) };
            }
        }
        if p != <I as IndexType>::max() {
            self.next[p.index()] = <I as IndexType>::max();
        }
        self.buffers.push((new_buf, new_buf_size));
    }
}
// https://github.com/alldatacenter/alldata/blob/2fc8f53f7f49f27354d65b02635af014f1b0aea1/kg/GraphScope/interactive_engine/executor/store/mcsr/src/mcsr.rs#L190