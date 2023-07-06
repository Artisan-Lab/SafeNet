unsafe fn get_activation_factory(library: PCSTR, name: &HSTRING) -> Result<IGenericFactory> {
    let function = delay_load::<DllGetActivationFactory>(library, s!("DllGetActivationFactory")).ok_or_else(Error::from_win32)?;
    let mut abi = std::mem::MaybeUninit::zeroed();
    function(std::mem::transmute_copy(name), abi.as_mut_ptr()).from_abi(abi)
}