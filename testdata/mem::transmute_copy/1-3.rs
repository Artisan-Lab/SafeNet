pub unsafe trait Abi: Sized {
    /// The type used to represent `Self` across FFI boundaries
    type Abi;

    /// Converts `self` to an Abi value
    ///
    /// Note: `Self::Abi` is only valid for however long `&self` lives for
    fn abi(&self) -> Self::Abi {
        // SAFETY: the `Abi` trait ensures that `Self` can be memcopied into `Self::Abi`
        unsafe { std::mem::transmute_copy(self) }
    }

    /// Converts an abi value to `Self` or fails if we can determine `abi` is not valid.
    ///
    /// # Safety
    ///
    /// `abi` must be in a state where it can be trivially transmuted into a valid value of type `Self`
    /// if `abi_is_possibly_valid` returns true.
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        if Self::abi_is_possibly_valid(&abi) {
            Ok(std::mem::transmute_copy(&abi))
        } else {
            Err(Error::OK)
        }
    }
}
// https://github.com/makepad/makepad/blob/17eef68bd1347eca1b920390642f9f44f25e4a6f/libs/windows/src/core/abi.rs#L24