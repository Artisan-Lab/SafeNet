pub unsafe extern "C" fn l10nregistry_addref(reg: *const GeckoL10nRegistry) {
    let raw = Rc::from_raw(reg);
    mem::forget(Rc::clone(&raw));
    mem::forget(raw);
}