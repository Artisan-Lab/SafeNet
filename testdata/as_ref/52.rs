pub fn to_maybe_jsx_import_source_module(&self) -> Option<String> {
    let compiler_options_value = self.json.compiler_options.as_ref()?;
    let compiler_options: CompilerOptions =
      serde_json::from_value(compiler_options_value.clone()).ok()?;
    match compiler_options.jsx.as_deref() {
      Some("react-jsx") => Some("jsx-runtime".to_string()),
      Some("react-jsxdev") => Some("jsx-dev-runtime".to_string()),
      _ => None,
    }
  }
/*
https://github.com/ISMAILBOUADDI/deno/blob/17d81ad2ef0001f862ba0a324e2f4d28f1cfdc78/cli/config_file.rs#L563
 */