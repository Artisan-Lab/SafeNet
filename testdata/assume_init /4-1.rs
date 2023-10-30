/*
https://github.com/denoland/deno/blob/d8e8e60f9f32ffce785b2efd71cd78b337a5352c/runtime/ops/os/sys_info.rs#L18
*/
pub fn loadavg() -> LoadAvg {
  #[cfg(target_os = "linux")]
  {
    use libc::SI_LOAD_SHIFT;

    let mut info = std::mem::MaybeUninit::uninit();
    // SAFETY: `info` is a valid pointer to a `libc::sysinfo` struct.
    let res = unsafe { libc::sysinfo(info.as_mut_ptr()) };
    if res == 0 {
      // SAFETY: `sysinfo` returns 0 on success, and `info` is initialized.
      let info = unsafe { info.assume_init() };
      (
        info.loads[0] as f64 / (1 << SI_LOAD_SHIFT) as f64,
        info.loads[1] as f64 / (1 << SI_LOAD_SHIFT) as f64,
        info.loads[2] as f64 / (1 << SI_LOAD_SHIFT) as f64,
      )
    } else {
      DEFAULT_LOADAVG
    }
  }
  #[cfg(any(
    target_vendor = "apple",
    target_os = "freebsd",
    target_os = "openbsd"
  ))]
  {
    let mut l: [f64; 3] = [0.; 3];
    // SAFETY: `&mut l` is a valid pointer to an array of 3 doubles
    if unsafe { libc::getloadavg(&mut l as *mut f64, l.len() as _) } < 3 {
      DEFAULT_LOADAVG
    } else {
      (l[0], l[1], l[2])
    }
  }
  #[cfg(target_os = "windows")]
  {
    DEFAULT_LOADAVG
  }
}