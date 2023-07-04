/*
From: https://github.com/denoland/deno/blob/aaabff710f756f7dc8651b8e92e4cc5147830e49/runtime/ops/runtime.rs#L62
*/
pub fn op_ppid() -> i64 {
  #[cfg(windows)]
  {
    // Adopted from rustup:
    // https://github.com/rust-lang/rustup/blob/1.21.1/src/cli/self_update.rs#L1036
    // Copyright Diggory Blake, the Mozilla Corporation, and rustup contributors.
    // Licensed under either of
    // - Apache License, Version 2.0
    // - MIT license
    use std::mem;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processthreadsapi::GetCurrentProcessId;
    use winapi::um::tlhelp32::CreateToolhelp32Snapshot;
    use winapi::um::tlhelp32::Process32First;
    use winapi::um::tlhelp32::Process32Next;
    use winapi::um::tlhelp32::PROCESSENTRY32;
    use winapi::um::tlhelp32::TH32CS_SNAPPROCESS;
    // SAFETY: winapi calls
    unsafe {
      // Take a snapshot of system processes, one of which is ours
      // and contains our parent's pid
      let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
      if snapshot == INVALID_HANDLE_VALUE {
        return -1;
      }

      let mut entry: PROCESSENTRY32 = mem::zeroed();
      entry.dwSize = mem::size_of::<PROCESSENTRY32>() as DWORD;

      // Iterate over system processes looking for ours
      let success = Process32First(snapshot, &mut entry);
      if success == 0 {
        CloseHandle(snapshot);
        return -1;
      }

      let this_pid = GetCurrentProcessId();
      while entry.th32ProcessID != this_pid {
        let success = Process32Next(snapshot, &mut entry);
        if success == 0 {
          CloseHandle(snapshot);
          return -1;
        }
      }
      CloseHandle(snapshot);

      // FIXME: Using the process ID exposes a race condition
      // wherein the parent process already exited and the OS
      // reassigned its ID.
      let parent_id = entry.th32ParentProcessID;
      parent_id.into()
    }