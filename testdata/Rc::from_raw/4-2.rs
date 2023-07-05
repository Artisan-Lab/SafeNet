pub fn downcast<T>(self) -> Result<Rc<T>> 
        where T: AnyObject + 'static
    {
        if TypeId::of::<T>() == self.0.type_id() {
            unsafe {
                let raw: *const dyn AnyObject = Rc::into_raw(self.0);
                Ok(Rc::from_raw(raw as *const T))
            }
        } else {
            Err(type_mismatch::<T>(self.0.type_name()))
        }
    }