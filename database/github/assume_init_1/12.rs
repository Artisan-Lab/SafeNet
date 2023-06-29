fn main() {
    let mut number = Box::<u32>::new_uninit();
    unsafe {
        number.as_mut_ptr().write(42); 
        let number: Box<u32> = number.assume_init(); 
        println!("Number: {}", *number); 
    }
}
