// impl<T> AsRef<T> for PgVarlena<T>
// where
//     T: Copy + Sized,
// {
    fn as_ref(&self) -> &T {
        unsafe {
            // safe: ptr will never be null
            let ptr = vardata_any(self.varlena.ptr) as *const T;
            ptr.as_ref().unwrap()
        }
    }
// }
/*
https://github.com/tcdi/pgrx/blob/6219d06e0c396229980854352fd969016d196a93/pgrx/src/datum/varlena.rs#L289
*/
