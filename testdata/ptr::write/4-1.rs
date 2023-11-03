pub fn new_source<T: SourceFuncs>(data: T) -> Source {
    unsafe {
        let mut funcs: GSourceFuncs = mem::zeroed();
        funcs.prepare = Some(prepare::<T>);
        funcs.check = Some(check::<T>);
        funcs.dispatch = Some(dispatch::<T>);
        funcs.finalize = Some(finalize::<T>);
        let mut funcs = Box::new(funcs);
        let source = g_source_new(&mut *funcs, mem::size_of::<SourceData<T>>() as u32);
        ptr::write(&mut (*(source as *mut SourceData<T>)).data, data);
        ptr::write(&mut (*(source as *mut SourceData<T>)).funcs, funcs);
        from_glib_full(source)
    }
}
// https://github.com/antoyo/relm/blob/439f5bf8317ea0dadfa7ea4dce05454d856e5a9f/src/core/source.rs#L56