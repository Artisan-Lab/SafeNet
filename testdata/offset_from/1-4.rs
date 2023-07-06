#[export_name = "__firefly_initialize_atom_table"]
pub unsafe extern "C-unwind" fn init(start: *const AtomData, end: *const AtomData) -> bool {
    if start == end {
        return true;
    }
    if start.is_null() || end.is_null() {
        return false;
    }

    debug_assert_eq!(
        ((end as usize) - (start as usize)) % mem::size_of::<AtomData>(),
        0,
        "invalid atom data range"
    );
    let len = end.offset_from(start);
    let data = slice::from_raw_parts::<'static, _>(start, len as usize);

    with_atom_table(|mut atoms| {
        atoms.extend(data);
    });

    true
}
//https://github.com/GetFirefly/firefly/blob/8e89bc7ec33cb8ffa9a60283c8dcb7ff62ead5fa/library/rt/src/term/atom/table.rs#L58