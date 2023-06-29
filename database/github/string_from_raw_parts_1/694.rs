extern "C" fn get_parameter_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, index: c_int, value: *mut c_float, value_str: *mut c_char) -> ::Result {
    unsafe {
        if !dsp_state.is_null() && !(*dsp_state).instance.is_null() {
            let mut tmp = ::std::ptr::null_mut();

            ffi::FMOD_DSP_GetUserData((*dsp_state).instance, &mut tmp);
            if !tmp.is_null() {
                let callbacks : &mut UserData = transmute(tmp);
                match callbacks.callbacks.get_param_callback {
                    Some(p) => {
                        let mut t_value = *value;
                        let l = ffi::strlen(value_str);
                        let tmp = String::from_raw_parts(value_str as *mut u8, l, l);

                        let ret = p(&from_state_ptr(::std::ptr::read(dsp_state as *const ffi::FMOD_DSP_STATE)),
                            index as i32,
                            &mut t_value,
                            &tmp);
                        *value = t_value;
                        ret
                    },
                    None => ::Result::Ok
                }
            } else {
              ::Result::Ok
            }
        } else {
           ::Result::Ok
        }
    }
}