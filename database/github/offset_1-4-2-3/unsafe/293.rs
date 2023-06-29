fn main() {
    let mut buffer: Vec<u8> = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa];

    unsafe {
        let ptr = buffer.as_mut_ptr();

        let a = ptr.offset(0) as *mut u32;
        *a = 0xdeadc0de;

        let b = ptr.offset(4) as *mut u16;
        *b = 0xbadf;
    }

    println!("{:x?}", buffer);
}