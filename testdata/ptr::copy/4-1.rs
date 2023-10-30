fn into_datum(self) -> Option<pg_sys::Datum> {
    unsafe {
        let boxed = PgBox::<pg_sys::BOX>::alloc0();
        std::ptr::copy(&self, boxed.as_ptr(), std::mem::size_of::<pg_sys::BOX>());
        boxed.into_datum()
    }
}
// https://github.com/tcdi/pgrx/blob/f048ce9acac12ad5e340cd7cd0f6ff419214b765/pgrx/src/datum/geo.rs#L34