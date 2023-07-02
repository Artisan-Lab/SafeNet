/*
https://github.com/microsoft/windows-rs/blob/a817607f4a48c891e4d598b40b8685c59d18743b/crates/libs/core/src/type.rs#L55
*/
unsafe fn from_abi(abi: std::mem::MaybeUninit<Self>) -> Result<Self> {
        Ok(abi.assume_init())
    }
