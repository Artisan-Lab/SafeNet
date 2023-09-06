// impl<T: Internable + AsRef<U>, U: ?Sized> AsRef<U> for Interned<T> {
    fn as_ref(&self) -> &U {
        let l = T::intern_cache().lock().unwrap();
        unsafe { mem::transmute::<&U, &U>(l.get(*self).as_ref()) }
    }
// }
//https://github.com/bjorn3/rust/blob/2f896da247e0ee8f0bef7cd7c54cfbea255b9f68/src/bootstrap/cache.rs#L100