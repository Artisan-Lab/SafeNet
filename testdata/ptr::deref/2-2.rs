pub fn root(&self) -> Option<DomRoot<T>> {
    unsafe { &*self.ptr.as_ptr() }.value.get().map(|ptr| unsafe {
        DomRoot::from_ref(&*ptr.as_ptr())
    })
}
//https://github.com/sdroege/servo/blob/0051597525797c4404cdc460389fe27780729626/components/script/dom/bindings/weakref.rs#L92