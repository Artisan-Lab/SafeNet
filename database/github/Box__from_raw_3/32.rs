fn __hyper_downcast<T: Io>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
    if self.__hyper_is::<T>() {
        // Taken from `std::error::Error::downcast()`.
        unsafe {
            let raw: *mut dyn Io = Box::into_raw(self);
            Ok(Box::from_raw(raw as *mut T))
        }
    } else {
        Err(self)
    }
}
/*
https://github.com/acfoltzer/hyper/blob/54eaf7fb1377dbb60c1b7a1f1e93388a58acd466/src/upgrade.rs#L287
*/