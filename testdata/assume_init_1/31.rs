unsafe fn new_zeroed(len: usize, allocator: Self::A) -> Self {
    Self(Box::new_zeroed_slice_in(len, allocator).assume_init())
}
/*
https://github.com/yutiansut/datafuse/blob/feda583559dfe76a6f8b12d8599be7843d27b929/src/common/hashtable/src/container.rs#L77
*/