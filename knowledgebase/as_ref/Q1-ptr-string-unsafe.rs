fn main() {
    let s = String::from("hello");
    let s_ref: &String = &s;
    let s_str_ref: &str = s_ref.as_ref();
    println!("{}", s_str_ref); 
}
