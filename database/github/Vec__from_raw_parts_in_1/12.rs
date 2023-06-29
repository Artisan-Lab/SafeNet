pub fn try_into_vec(self) -> Result<Vec<T, A>, Self> {
    match self.capacity() {
        Some(capacity) if capacity > N => unsafe {
            let (meta, data, alloc) = self.into_raw_parts_with_alloc();
            Ok(Vec::from_raw_parts_in(data.ptr.as_ptr(), meta.len(), capacity, alloc))
        },
        _ => Err(self)
    }
}
/*
https://github.com/timothee-haudebourg/calf-vec/blob/49e4d649c32cee0fd8e02ce70692663914647cb2/src/generic.rs#L286
*/