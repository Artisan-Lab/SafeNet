fn main(){
    let x = {
        let mut x: [MaybeUninit<Box<u32>>; SIZE] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for i in 0..SIZE {
            x[i] = MaybeUninit::new(Box::new(i as u32));
        }
        unsafe { mem::transmute::<_, [Box<u32>; SIZE]>(x) }
    };    
}