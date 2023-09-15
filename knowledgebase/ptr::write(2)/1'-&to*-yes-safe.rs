fn main() {
    let mut value = 42;
    let mut new_value = 100;
    
    let old_value = std::mem::replace(&mut value, new_value);
    
    println!("Old Value: {}", old_value);
    println!("New Value: {}", value);
}
