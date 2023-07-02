fn main() {
    let x: Vec<u8> = vec![72, 101, 108, 108, 111]; 
    let y: String = x.into_iter().map(|byte| byte as char).collect();

    println!("String data: {}", );
}
