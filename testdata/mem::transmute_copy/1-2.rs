pub fn vec_cast<T, U>(input: Vec<T>) -> Vec<U> {
    input.into_iter().map(|e| unsafe { std::mem::transmute_copy(&e) }).collect()
}

// https://github.com/Hoverbear/xous-rust/blob/03385b063c0d999f13058fd23aceced436e84182/library/alloc/benches/vec.rs#L512