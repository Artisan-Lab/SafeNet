fn main() {
    let mut v = Box::<u32>::new_uninit();  
    let v: Box<u32> = unsafe {
        v.as_mut_ptr().write(3); 
        v.assume_init()  
    }; 
}
