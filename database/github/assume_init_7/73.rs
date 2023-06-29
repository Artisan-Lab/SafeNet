use std::mem::{self, MaybeUninit};
pub fn new<T: Rng>(rng: &mut T) -> Self {
    let mut random_vecs: [MaybeUninit<Vec3>; POINT_COUNT] =
        unsafe { MaybeUninit::uninit().assume_init() };
}
/*
https://github.com/mbStavola/ray_tracer/blob/bdade6f82b5f9f463ae4fc537b28891aa13e5d07/src/perlin.rs#L23
*/