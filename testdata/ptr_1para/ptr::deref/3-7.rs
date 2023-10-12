fn downgrade(&self) -> WeakRef<Self> {
    unsafe {
        let object = self.reflector().get_jsobject().get();
        let mut slot = UndefinedValue();
        JS_GetReservedSlot(object, DOM_WEAK_SLOT, &mut slot);
        let mut ptr = slot.to_private() as *mut WeakBox<Self>;
        if ptr.is_null() {
            trace!("Creating new WeakBox holder for {:p}.", self);
            ptr = Box::into_raw(Box::new(WeakBox {
                count: Cell::new(1),
                value: Cell::new(Some(ptr::NonNull::from(self))),
            }));
            let val = PrivateValue(ptr as *const c_void);
            JS_SetReservedSlot(object, DOM_WEAK_SLOT, &val);
        }
        let box_ = &*ptr;
        assert!(box_.value.get().is_some());
        let new_count = box_.count.get() + 1;
        trace!("Incrementing WeakBox refcount for {:p} to {}.",
                   self,
                   new_count);
        box_.count.set(new_count);
        WeakRef {
            ptr: ptr::NonNull::new_unchecked(ptr),
        }
    }
}

//https://github.com/gterzian/servo/blob/8a96bf688aef961eb11abc209c8ac37ee87b439f/components/script/dom/bindings/weakref.rs#L70