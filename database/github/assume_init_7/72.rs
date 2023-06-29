use std::mem::{self, MaybeUninit};
fn flip_rectangle(&mut self, rectangle: &Rectangle) {
    unsafe {
        let mut begin = 0;
        let mut edges: [MaybeUninit<Vec2>; 4] = MaybeUninit::uninit().assume_init();
    }
}
/*
https://github.com/terry-u16/ahc014/blob/2a1430a18ee3759f6203006e0fc70255018beedc/src/bin/a.rs#L554
*/