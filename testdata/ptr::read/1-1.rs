unsafe fn read_value_from(p: *const u128, ty: Type) -> DataValue {
    match ty {
        ir::types::I8 => DataValue::I8(ptr::read(p as *const i8)),
        ir::types::I16 => DataValue::I16(ptr::read(p as *const i16)),
        ir::types::I32 => DataValue::I32(ptr::read(p as *const i32)),
        ir::types::I64 => DataValue::I64(ptr::read(p as *const i64)),
        ir::types::F32 => DataValue::F32(ptr::read(p as *const Ieee32)),
        ir::types::F64 => DataValue::F64(ptr::read(p as *const Ieee64)),
        _ if ty.is_bool() => DataValue::B(ptr::read(p as *const bool)),
        _ if ty.is_vector() && ty.bytes() == 16 => {
            DataValue::V128(ptr::read(p as *const [u8; 16]))
        }
        _ => unimplemented!(),
    }
}
// https://github.com/sunfishcode/wasmtime/blob/696dca4d01abe12619a72acaccd7d20b3b15485c/cranelift/filetests/src/function_runner.rs#L247