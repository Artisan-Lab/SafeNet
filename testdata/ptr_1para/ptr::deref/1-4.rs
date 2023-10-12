unsafe fn copy_grouping(group: *const libc::c_char, vm: &VirtualMachine) -> PyListRef {
    let mut group_vec: Vec<PyObjectRef> = Vec::new();
    if group.is_null() {
        return vm.ctx.new_list(group_vec);
    }

    let mut ptr = group;
    while ![0, libc::c_char::MAX].contains(&*ptr) {
        let val = vm.ctx.new_int(*ptr);
        group_vec.push(val.into());
        ptr = ptr.add(1);
    }
    // https://github.com/python/cpython/blob/677320348728ce058fa3579017e985af74a236d4/Modules/_localemodule.c#L80
    if !group_vec.is_empty() {
        group_vec.push(vm.ctx.new_int(0).into());
    }
    vm.ctx.new_list(group_vec)
}
// https://github.com/lawchingman/RustPython/blob/87728c44527272ece739cddbb4942ad7e176dd79/stdlib/src/locale.rs#L44