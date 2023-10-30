fn module_resolve_callback<'s>(
    context: v8::Local<'s, v8::Context>,
    specifier: v8::Local<'s, v8::String>,
    import_assertions: v8::Local<'s, v8::FixedArray>,
    referrer: v8::Local<'s, v8::Module>,
  ) -> Option<v8::Local<'s, v8::Module>> {
    // SAFETY: `CallbackScope` can be safely constructed from `Local<Context>`
    let scope = &mut unsafe { v8::CallbackScope::new(context) };

    let module_map =
      // SAFETY: We retrieve the pointer from the slot, having just set it a few stack frames up
      unsafe { scope.get_slot::<*const Self>().unwrap().as_ref().unwrap() };

    let referrer_global = v8::Global::new(scope, referrer);

    let referrer_info = module_map
      .get_info(&referrer_global)
      .expect("ModuleInfo not found");
    let referrer_name = referrer_info.name.as_str();

    let specifier_str = specifier.to_rust_string_lossy(scope);

    let assertions = parse_import_assertions(
      scope,
      import_assertions,
      ImportAssertionsKind::StaticImport,
    );
    let maybe_module = module_map.resolve_callback(
      scope,
      &specifier_str,
      referrer_name,
      assertions,
    );
    if let Some(module) = maybe_module {
      return Some(module);
    }

    let msg = format!(
      r#"Cannot resolve module "{specifier_str}" from "{referrer_name}""#
    );
    throw_type_error(scope, msg);
    None
  }

// https://github.com/DjDeveloperr/deno/blob/b319fa7f4965af3d3d576ea528248a31c96a4053/core/modules/map.rs#L600