pub fn downcast_rc<'a, T: Resource>(self: &'a Rc<Self>) -> Option<&'a Rc<T>> {
    if self.is::<T>() {
        let ptr = self as *const Rc<_> as *const Rc<T>;
        // TODO(piscisaureus): safety comment
        #[allow(clippy::undocumented_unsafe_blocks)]
        Some(unsafe { &*ptr })
    } else {
        None
    }
}
// https://github.com/dsherret/deno/blob/efa7c19890f58d9d446477e03e460b1190023c85/core/resources.rs#L207