pub fn to_u32(val: [u8; 4]) -> u32 {
    let reversed = [val[3], val[2], val[1], val[0]];

    unsafe {
        std::mem::transmute::<[u8; 4], u32>(reversed)
    }
}
