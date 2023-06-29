pub fn get_mut(&mut self, index: OpIndex) -> &mut GosValue {
    unsafe { self.vec.get_unchecked_mut(index as usize) }
}
/*
https://github.com/oxfeeefeee/goscript/blob/a116777ba750af87c975624fa348b721896677c4/vm/src/stack.rs#L35
*/