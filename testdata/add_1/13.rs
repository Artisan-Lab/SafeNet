fn build_sparse_table<T: Ord>(&mut self, a: *const T, n: usize) {
    let mut height = 0;
    while (1 << height) < self.blocks {
        height += 1;
    }
    self.table
        .resize_with(self.blocks * (height + 1), Default::default);
    let mut ptr = self.table.as_mut_ptr();
    for idx in 0..self.blocks {
        let mut best_idx = idx * Self::BLOCK_SIZE;
        let bound = min(n, (idx + 1) * Self::BLOCK_SIZE);
        for j in best_idx + 1..bound {
            best_idx = Self::min_index(a, best_idx, j);
        }
        unsafe {
            *ptr.add(idx) = best_idx;
        }
    }
    for i in 0..height {
        let t = 1 << i;
        unsafe {
            ptr = ptr.add(self.blocks);
            ptr.copy_from_nonoverlapping(ptr.sub(self.blocks), self.blocks);
            for i in 0..self.blocks - t {
                *ptr.add(i) = Self::min_index(a, *ptr.add(i), *ptr.add(i + t));
            }
        }
    }
}


// https://github.com/zimpha/algorithmic-library/blob/804b5859e96ed3240ef7ecac5fabb5fde88e15fc/rust/src/data_structure/bit_direct_rmq.rs#L102