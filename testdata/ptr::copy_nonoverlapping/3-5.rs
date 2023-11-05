fn alloc(&mut self) -> usize {
    let ret = self.head;
    if ret == self.data.len() {
        let curr_len = self.data.len();
        if curr_len == self.data.capacity() {
            let extra = max(128, curr_len);
            let r = unsafe { __wbindgen_externref_table_grow(extra) };
            if r == -1 {
                internal_error("table grow failure")
            }
            if self.base == 0 {
                self.base = r as usize;
            } else if self.base + self.data.len() != r as usize {
                internal_error("someone else allocated table entries?")
            }

            // poor man's `try_reserve_exact` until that's stable
            unsafe {
                let new_cap = self.data.capacity() + extra;
                let size = mem::size_of::<usize>() * new_cap;
                let align = mem::align_of::<usize>();
                let layout = match Layout::from_size_align(size, align) {
                    Ok(l) => l,
                    Err(_) => internal_error("size/align layout failure"),
                };
                let ptr = alloc::alloc(layout) as *mut usize;
                if ptr.is_null() {
                    internal_error("allocation failure");
                }
                ptr::copy_nonoverlapping(self.data.as_ptr(), ptr, self.data.len());
                let new_vec = Vec::from_raw_parts(ptr, self.data.len(), new_cap);
                let mut old = mem::replace(&mut self.data, new_vec);
                old.set_len(0);
            }
        }

        // custom condition to ensure `push` below doesn't call `reserve` in
        // optimized builds which pulls in lots of panic infrastructure
        if self.data.len() >= self.data.capacity() {
            internal_error("push should be infallible now")
        }
        self.data.push(ret + 1);
    }
    // https://github.com/rustwasm/wasm-bindgen/blob/4aae6b38e6bed37d3b908f6e99611549840ab395/src/externref.rs#L62