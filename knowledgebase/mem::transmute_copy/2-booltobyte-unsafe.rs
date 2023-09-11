// use std::mem;

fn bool_to_byte(value: bool) -> u8 {
    unsafe {
        let bool_array: [bool; 1] = [value];
        let byte_array: [u8; 1] = mem::transmute_copy(&bool_array);
        byte_array[0]
    }
}

// fn main() {
//     let result = bool_to_byte(true);
//     println!("{}", result);
// }
