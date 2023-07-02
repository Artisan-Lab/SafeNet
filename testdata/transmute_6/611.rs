fn main() {
    let u: &[u8] = &[49, 50, 51];

    unsafe {
        assert!(u == std::mem::transmute::<&str, &[u8]>("123"));
    }
}
/*
https://github.com/vladikoff/rust-by-example/blob/74dad23363fb90bbbbb31e1490cf75327f053231/src/unsafe.md?plain=1#L40
*/