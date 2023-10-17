fn main() {
    let x: Vec<u8> = vec![72, 101, 108, 108, 111]; 
    let y: String = unsafe { std::mem::transmute(x) };

    // println!("String data: {}", y);
}
