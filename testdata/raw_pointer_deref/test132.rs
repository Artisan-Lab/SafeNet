fn main() {
    let arr = [1, 2, 3];
    unsafe {
        let mut ptr = arr.as_ptr();
        let end = ptr.add(arr.len());

        while ptr < end {
            println!("Value of element: {}", *ptr);
            ptr = ptr.offset(1);
        }
    }
}