fn replace_with_callback<F: FnMut() -> String>(
    original: &str,
  ) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    result.push_str(unsafe { original.get_unchecked(last_end..original.len()) });
  }
/*
https://github.com/tauri-apps/tauri/blob/38d0bed8ebb6c5872fc54903051d146d386b721e/core/tauri/src/manager.rs#L163
*/