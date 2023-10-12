unsafe fn offset_from<T>(to: *const T, from: *const T) -> usize {
    to.offset_from(from) as usize
}

// https://github.com/rust-lang/hashbrown/blob/3d2d1638d90053cb7d6a96090bc7c2bd2fd10d71/src/raw/mod.rs#L64