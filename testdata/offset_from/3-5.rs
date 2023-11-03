pub fn program_counter(&self) -> usize {
    // Safety: this is just subtraction of pointers, it is safe to do.
    unsafe {
        self.instruction_pointer
            .offset_from(self.contract.bytecode.as_ptr()) as usize
    }
}
//https://github.com/bluealloy/revm/blob/10f81ba1b3cd6171c25d3346f1800bbb04f5ead1/crates/interpreter/src/interpreter.rs#L122