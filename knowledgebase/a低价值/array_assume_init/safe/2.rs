fn main(){
    let mut a:[u8; 4]=[0,1,2,4];
    let b = u32::from_le_bytes(a);
    println!("{}",b);
}
