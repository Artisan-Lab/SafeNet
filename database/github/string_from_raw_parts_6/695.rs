/*
From: https://github.com/0xrawsec/sudocker/blob/90fee6579bcc1b6ac295cb2c595be348f51a4eb3/src/main.rs#L24
*/
fn strerror(e: i32) -> String {
    let str_err = unsafe { libc::strerror(e) };
    let str_len = strlen(str_err);
    unsafe { String::from_raw_parts(str_err as *mut u8, str_len, str_len) }
}