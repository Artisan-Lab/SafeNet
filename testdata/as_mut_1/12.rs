impl<T, AllocatedBy: WhoAllocated> DerefMut for PgBox<T, AllocatedBy> {
    #[track_caller]
    fn deref_mut(&mut self) -> &mut T {
        match self.ptr.as_mut() {
            Some(ptr) => unsafe { ptr.as_mut() },
            None => panic!("Attempt to dereference null pointer during DerefMut of PgBox"),
        }
    }
}

// https://github.com/tcdi/pgrx/blob/6219d06e0c396229980854352fd969016d196a93/pgrx/src/pgbox.rs#L387