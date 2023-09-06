// impl Hash for ThreadReceiverInner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.ptr.as_ref() as *const _ as usize).hash(state);
    }
// }

/*
https://github.com/fschutt/azul/blob/b40a2ad6e22382842ce6ccd631b94fb5a6e1742d/azul-core/src/task.rs#L979
*/