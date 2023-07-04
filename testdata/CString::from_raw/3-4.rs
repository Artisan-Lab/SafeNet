pub fn call_path<S: Serialize, T>(
    path: &EffectPath,
    key: &[u8; 32],
    args: CreateArgs<S>,
) -> Result<T, CompilationError>
where
    T: for<'a> Deserialize<'a> + JsonSchema,
{
    let path = serde_json::to_string(path).map_err(CompilationError::SerializationError)?;
    let s = serde_json::to_value(args)
        .map_err(CompilationError::SerializationError)?
        .to_string();
    let l = s.len();
    let p = unsafe {
        sapio_v1_wasm_plugin_create_contract(
            path.as_ptr() as i32,
            path.len() as i32,
            key.as_ptr() as i32,
            s.as_ptr() as i32,
            l as i32,
        )
    };
    if p != 0 {
        let cs = unsafe { CString::from_raw(p as *mut c_char) };
        let res: Result<T, String> = serde_json::from_slice(cs.as_bytes())
            .map_err(CompilationError::DeserializationError)?;
        res.map_err(CompilationError::ModuleCompilationErrorUnsendable)
    } else {
        Err(CompilationError::InternalModuleError("Unknown".into()))
    }
}