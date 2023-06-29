#![allow(unused)]

fn main() {
    let tls = unsafe {
        std::mem::transmute::<_, [u8; 32]>([stack_tos, secondary as u64, 0u64, 0u64])
    };
}

