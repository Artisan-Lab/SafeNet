pub fn has_class<T>(
    item: T,
    name: &Atom,
    case_sensitivity: CaseSensitivity,
    getter: ClassOrClassList<T>,
) -> bool {
    unsafe {
        let mut class: *mut nsAtom = ptr::null_mut();
        let mut list: *mut *mut nsAtom = ptr::null_mut();
        let length = getter(item, &mut class, &mut list);
        match length {
            0 => false,
            1 => case_sensitivity.eq_atom(name, WeakAtom::new(class)),
            n => {
                let classes = slice::from_raw_parts(list, n as usize);
                match case_sensitivity {
                    CaseSensitivity::CaseSensitive => {
                        classes.iter().any(|ptr| &**name == WeakAtom::new(*ptr))
                    }
                    CaseSensitivity::AsciiCaseInsensitive => {
                        classes.iter().any(|ptr| name.eq_ignore_ascii_case(WeakAtom::new(*ptr)))
                    }
                }
            }
        }
    }
}

//https://github.com/moveman/servo/blob/726a1854b08b7809dbe31fa57c186de39bd5ca42/components/style/gecko/snapshot_helpers.rs#L40