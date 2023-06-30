pub unsafe fn from_raw(buf: Vec<u8>, reg_type: co::REG) -> RegistryValue {
    match reg_type {
        co::REG::NONE => RegistryValue::None,
        co::REG::DWORD => RegistryValue::Dword(
            u32::from_ne_bytes(unsafe {
                *std::mem::transmute::<_, *const [u8; 4]>(buf.as_ptr())
            })
        ),
        co::REG::QWORD => RegistryValue::Qword(
            u64::from_ne_bytes(unsafe {
                *std::mem::transmute::<_, *const [u8; 8]>(buf.as_ptr())
            })
        ),
        co::REG::SZ => {
            let (_, vec16, _) = unsafe { buf.align_to::<u16>() };
            RegistryValue::Sz(WString::from_wchars_slice(&vec16).to_string())
        },
        co::REG::EXPAND_SZ => {
            let (_, vec16, _) = unsafe { buf.align_to::<u16>() };
            RegistryValue::Sz(WString::from_wchars_slice(&vec16).to_string())
        },
        co::REG::MULTI_SZ => {
            let (_, vec16, _) = unsafe { buf.align_to::<u16>() };
            RegistryValue::MultiSz(parse_multi_z_str(vec16.as_ptr()))
        },
        co::REG::BINARY => RegistryValue::Binary(buf),
        _ => RegistryValue::None, // other types not implemented yet
    }
}
/*
https://github.com/rodrigocfd/winsafe/blob/207a1e84a3e396cccca55d060cc6611553f046d1/src/kernel/enums.rs#L143
*/