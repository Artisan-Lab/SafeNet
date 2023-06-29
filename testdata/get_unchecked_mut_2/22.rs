unsafe fn array_axcpy<Status, T>(
    _: Status,
    y: &mut [Status::Value],
    x: &[T],
) where
    Status: InitStatus<T>,
    T: Scalar + Zero + ClosedAdd + ClosedMul,
{
    for i in 0..len {
        let y = Status::assume_init_mut(y.get_unchecked_mut(i * stride1));
    }
}
/*
https://github.com/dimforge/nalgebra/blob/5baf86b3111858cdecb6518f21ed3b2c579d04f5/src/base/blas_uninit.rs#L46
*/