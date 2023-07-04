use std::ffi::CString;

fn main() {
    let cstring = CString::new("hello").expect("Failed to create CString");
    let mut bytes = cstring.into_bytes();

    bytes.iter_mut().for_each(|byte| *byte += 1);
    bytes.push(b'\0');

    // Convert the modified byte vector back to CString
    let modified_cstring = CString::from_vec_with_nul(bytes);

    println!("Modified: {:?}", modified_cstring);
}
