use std::ffi::CStr;

fn process_cstr(cstr_ref: &str) {
    
    let cstr_ptr = cstr_ref.as_ptr();
    let cstr = unsafe { CStr::from_ptr(cstr_ptr) };
    let str_value = cstr.to_string_lossy().into_owned();

    println!("Value: {}", str_value);

}

fn main() {
    let str_value = "Hello, World!";
    process_cstr(str_value);
}
