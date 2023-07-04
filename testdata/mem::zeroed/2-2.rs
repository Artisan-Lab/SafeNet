/*
https://github.com/JetBrains/intellij-community/blob/39c93a60b891c4085ddc0ea0388d8c52f842ee13/native/XPlatLauncher/src/lib.rs#L266
*/

fn win_user_profile_dir() -> Result<String> {
    let token = Foundation::HANDLE(-4);  // as defined in `GetCurrentProcessToken()`; Windows 8+/Server 2012+
    let mut buf: [u16; Foundation::MAX_PATH as usize] = unsafe { std::mem::zeroed() };
    let mut size = buf.len() as u32;
    debug!("Calling GetUserProfileDirectoryW({:?})", token);
    let result: Foundation::BOOL = unsafe {
        Shell::GetUserProfileDirectoryW(token, PWSTR::from_raw(buf.as_mut_ptr()), std::ptr::addr_of_mut!(size))
    };
    debug!("  result: {:?}, size: {}", result, size);
    if result == Foundation::TRUE {
        Ok(String::from_utf16(&buf[0..(size - 1) as usize])?)
    } else {
        bail!("GetUserProfileDirectoryW(): {:?}", unsafe { Foundation::GetLastError() })
    }
}