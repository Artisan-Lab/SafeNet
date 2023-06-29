use std::ptr::NonNull;
impl std::fmt::Debug for ObjPtrWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = match NonNull::new(self.0) {
            Some(ptr) => ptr,
            None => return write!(f, "{:?}", None as Option<i32>),
        };impl std::fmt::Debug for ObjPtrWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = match NonNull::new(self.0) {
            Some(ptr) => ptr,
            None => return write!(f, "{:?}", None as Option<i32>),
        };

        let kind = unsafe { ptr.as_ref().kind };
    }
}
}
}