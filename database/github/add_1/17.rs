pub fn bytecodes(&self) -> Vec<u64> {
    let ptr = self.get_bytecode();
    let size = self.bytecode_size();
    let mut insns = vec![];
    for i in 0..size {
        unsafe { insns.push(*ptr.add(i as usize)) };
    }
    insns
}
