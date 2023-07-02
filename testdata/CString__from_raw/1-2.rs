pub fn config_board<S: AsRef<str>>(&self, config: S) -> Result<String> {
    let config = CString::new(config.as_ref())?;
    let mut response_len = 0;
    let mut result_char_buffer: [c_char; 8192] = [0; 8192];

    let config = config.into_raw();
    let (res, response) = unsafe {
        let res = board_controller::config_board(
            config,
            result_char_buffer.as_mut_ptr(),
            &mut response_len,
            self.board_id as c_int,
            self.json_brainflow_input_params.as_ptr(),
        );
        let _ = CString::from_raw(config);
        let resp = CStr::from_ptr(result_char_buffer.as_ptr());

        (res, resp)
    };
    check_brainflow_exit_code(res)?;
    Ok(response
        .to_str()?
        .to_string())
}
