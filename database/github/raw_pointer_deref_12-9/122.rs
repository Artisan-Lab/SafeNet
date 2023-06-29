fn main() {
    let mut num = 5;
    unsafe {
        let ptr = &mut num as *mut i32;
        *ptr += 10;

        if *ptr > 10 {
            println!("Greater than 10");
        } else {
            println!("Less than or equal to 10");
        }

        let raw = ptr as *const i32;
        println!("Raw pointer: {:?}", raw);
        println!("Dereferenced pointer: {:?}", *raw);
    }
}
