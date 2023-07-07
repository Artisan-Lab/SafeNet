pub fn each_class<F, T>(
    item: T,
    mut callback: F,
    getter: ClassOrClassList<T>,
)
    where
        F: FnMut(&Atom)
{
    unsafe {
        let mut class: *mut nsAtom = ptr::null_mut();
        let mut list: *mut *mut nsAtom = ptr::null_mut();
        let length = getter(item, &mut class, &mut list);
        match length {
            0 => {}
            1 => Atom::with(class, callback),
            n => {
                let classes = slice::from_raw_parts(list, n as usize);
                for c in classes {
                    Atom::with(*c, &mut callback)
                }
            }
        }
    }
}

//https://github.com/moveman/servo/blob/726a1854b08b7809dbe31fa57c186de39bd5ca42/components/style/gecko/snapshot_helpers.rs#L40