fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> Result<Self> {
    match value {
        #[cfg(feature = "luau")]
        #[rustfmt::skip]
        Value::Vector(v) if N == crate::types::Vector::SIZE => unsafe {
            use std::{mem, ptr};
            let mut arr: [mem::MaybeUninit<T>; N] = mem::MaybeUninit::uninit().assume_init();
            ptr::write(arr[0].as_mut_ptr() , T::from_lua(Value::Number(v.x() as _), _lua)?);
            ptr::write(arr[1].as_mut_ptr(), T::from_lua(Value::Number(v.y() as _), _lua)?);
            ptr::write(arr[2].as_mut_ptr(), T::from_lua(Value::Number(v.z() as _), _lua)?);
            #[cfg(feature = "luau-vector4")]
            ptr::write(arr[3].as_mut_ptr(), T::from_lua(Value::Number(v.w() as _), _lua)?);
            Ok(mem::transmute_copy(&arr))
        },
        Value::Table(table) => {
            let vec = table.sequence_values().collect::<Result<Vec<_>>>()?;
            vec.try_into()
                .map_err(|vec: Vec<T>| Error::FromLuaConversionError {
                    from: "Table",
                    to: "Array",
                    message: Some(format!("expected table of length {}, got {}", N, vec.len())),
                })
        }
        _ => Err(Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Array",
            message: Some("expected table".to_string()),
        }),
    }
}

// https://github.com/khvzak/mlua/blob/925a2816cc8d25aececb51534adcbbe0de1e23b3/src/conversion.rs#L596