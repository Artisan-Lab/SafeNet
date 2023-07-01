fn foo(x:Box<i32>) -> Box<u32> {
    Box::new(*x as u32)
}

fn main() {
    // Convert a boxed i32 to a boxed u32
    let x1: Box<i32> = Box::new(42);
    let x2 = foo(x1);
    println!("Boxed u32: {}", *x2);
}