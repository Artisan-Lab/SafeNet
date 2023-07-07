pub unsafe extern "C" fn fluent_bundle_add_resource(
    bundle: &mut FluentBundleRc,
    r: &FluentResource,
    allow_overrides: bool,
    ret_errors: &mut ThinVec<nsCString>,
) {
    // we don't own the resource
    let r = mem::ManuallyDrop::new(Rc::from_raw(r));

    if allow_overrides {
        bundle.add_resource_overriding(Rc::clone(&r));
    } else if let Err(errors) = bundle.add_resource(Rc::clone(&r)) {
        append_fluent_errors_to_ret_errors(ret_errors, &errors);
    }
}