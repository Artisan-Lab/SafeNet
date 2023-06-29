impl Traveler<U4> for U4 {
    fn read<I>(seq: &mut I, _constants: Option<&ConstantPool>) -> U4
    where
        I: Iterator<Item = u8>,
    {
        let u0 = seq.next().unwrap();
        let u1 = seq.next().unwrap();
        let u2 = seq.next().unwrap();
        let u3 = seq.next().unwrap();
        let u = [u3, u2, u1, u0];
        unsafe { transmute::<[u8; 4], u32>(u) }
    }
}
               
