pub fn factory<C: crate::RuntimeName, I: crate::ComInterface>() -> crate::Result<I> {
    let mut factory: Option<I> = None;
    let name = crate::HSTRING::from(C::NAME);

    let code = if let Some(function) = unsafe { delay_load::<RoGetActivationFactory>(crate::s!("combase.dll"), crate::s!("RoGetActivationFactory")) } {
        unsafe {
            let mut code = function(std::mem::transmute_copy(&name), &I::IID, &mut factory as *mut _ as *mut _);

            // If RoGetActivationFactory fails because combase hasn't been loaded yet then load combase
            // automatically so that it "just works" for apartment-agnostic code.
            if code == CO_E_NOTINITIALIZED {
                if let Some(mta) = delay_load::<CoIncrementMTAUsage>(crate::s!("ole32.dll"), crate::s!("CoIncrementMTAUsage")) {
                    let mut cookie = std::ptr::null_mut();
                    let _ = mta(&mut cookie);
                }

                // Now try a second time to get the activation factory via the OS.
                code = function(std::mem::transmute_copy(&name), &I::IID, &mut factory as *mut _ as *mut _);
            }

            code
        }
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    };

    // If this succeeded then return the resulting factory interface.
    if code.is_ok() {
        return code.and_some(factory);
    }

    // If not, first capture the error information from the failure above so that we
    // can ultimately return this error information if all else fails.
    let original: crate::Error = code.into();

    // Now attempt to find the factory's implementation heuristically.
    if let Some(i) = search_path(C::NAME, |library| unsafe { get_activation_factory(library, &name) }) {
        i.cast()
    } else {
        Err(original)
    }
}
// https://github.com/microsoft/windows-rs/blob/7b424c2590ac21b47e8ac16b31460fd625e28122/crates/libs/core/src/imp/factory_cache.rs#L58