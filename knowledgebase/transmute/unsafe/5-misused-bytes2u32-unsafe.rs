/*
From: https://doc.rust-lang.org/std/mem/fn.transmute.html
*/

fn main() {
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];
    let num = unsafe {
        std::mem::transmute::<[u8; 4], u32>(raw_bytes)
    };
    assert_eq!(num, 0x12345678);
}
