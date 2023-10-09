fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        if inner.rc.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }
    }

/*
https://github.com/rust-lang-cn/nomicon-zh-Hans/blob/3295769d85dd1f1ece0edb9e31ec818c8569e800/src/arc-mutex/arc-drop.md?plain=1#L69
 */