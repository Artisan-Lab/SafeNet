pub extern "C" fn orchard_wallet_load_bundle(
    bundle: *const Bundle<Authorized, Amount>,
) -> bool {
    let bundle = unsafe { bundle.as_ref() }.expect("bundle pointer may not be null.");
    true
}

