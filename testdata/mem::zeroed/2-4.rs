/*
https://github.com/denogdev/denog/blob/29dade113e7d7e7d1ececace506e8b1a47eeca95/runtime/ops/tty.rs#L259
*/

pub fn console_size(
  std_file: &std::fs::File,
) -> Result<ConsoleSize, std::io::Error> {
  #[cfg(windows)]
  {
    use std::os::windows::io::AsRawHandle;
    let handle = std_file.as_raw_handle();

    // SAFETY: winapi calls
    unsafe {
      let mut bufinfo: winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO =
        std::mem::zeroed();

      if winapi::um::wincon::GetConsoleScreenBufferInfo(handle, &mut bufinfo)
        == 0
      {
        return Err(Error::last_os_error());
      }
      Ok(ConsoleSize {
        cols: bufinfo.dwSize.X as u32,
        rows: bufinfo.dwSize.Y as u32,
      })
    }
  }

  #[cfg(unix)]
  {
    use std::os::unix::io::AsRawFd;

    let fd = std_file.as_raw_fd();
    // SAFETY: libc calls
    unsafe {
      let mut size: libc::winsize = std::mem::zeroed();
      if libc::ioctl(fd, libc::TIOCGWINSZ, &mut size as *mut _) != 0 {
        return Err(Error::last_os_error());
      }
      Ok(ConsoleSize {
        cols: size.ws_col as u32,
        rows: size.ws_row as u32,
      })
    }
  }
}