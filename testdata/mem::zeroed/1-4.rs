/*
From: https://github.com/tauri-apps/tauri/blob/3065c8aea375535763e1532951c4057a426fce80/tooling/cli/src/helpers/flock.rs#L230
*/

fn is_on_nfs_mount(path: &Path) -> bool {
    use std::ffi::CString;
    use std::mem;
    use std::os::unix::prelude::*;

    let path = match CString::new(path.as_os_str().as_bytes()) {
      Ok(path) => path,
      Err(_) => return false,
    };

    unsafe {
      let mut buf: libc::statfs = mem::zeroed();
      let r = libc::statfs(path.as_ptr(), &mut buf);

      r == 0 && buf.f_type as u32 == libc::NFS_SUPER_MAGIC as u32
    }
  }
