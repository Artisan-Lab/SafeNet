fn main() {
    let mut s = String::from("hello");
    let l = s.len();
    for i in 0..l {
        let e = s.remove(i);
        s.insert(i,char::from_u32(e as u32 + 1).unwrap());
    }
    println!("s: {}", s);
}
