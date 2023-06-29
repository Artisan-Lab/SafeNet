
pub fn vec_to_gpu_vec<T>(v: Vec<T>) -> GpuVec<T> {
    let (ptr, length, capacity) = v.into_raw_parts();
    unsafe { Vec::from_raw_parts_in(ptr, length, capacity, GpuAllocator) }
}
/*
https://github.com/andrewmilson/ministark/blob/4e7d909060e992cab49af043d6bff4eb159c0279/src/utils.rs#L466
*/