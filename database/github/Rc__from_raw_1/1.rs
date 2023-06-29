pub fn cleanup(&mut self) {
    self.prepare_for_cleanup();

    let state_ptr = self.v8_isolate.get_data(STATE_DATA_OFFSET);
    // SAFETY: We are sure that it's a valid pointer for whole lifetime of
    // the runtime.
    _ = unsafe { Rc::from_raw(state_ptr as *const RefCell<JsRuntimeState>) };

    let module_map_ptr = self.v8_isolate.get_data(MODULE_MAP_DATA_OFFSET);
    // SAFETY: We are sure that it's a valid pointer for whole lifetime of
    // the runtime.
    _ = unsafe { Rc::from_raw(module_map_ptr as *const RefCell<ModuleMap>) };

    self.state.borrow_mut().destroy_all_realms();

    debug_assert_eq!(Rc::strong_count(&self.state), 1);
  }
  /*
  https://github.com/denoland/deno/blob/4a18c761351dccb146973793cf22e6efffff18bf/core/runtime/jsruntime.rs#L140
  */