pub fn new(prog: &[u8], helpers: &HashMap<u32, ebpf::Helper>, use_mbuff: bool,
    update_data_ptr: bool) -> Result<JitMemory<'a>, Error> {
let contents: &mut[u8];
let mut raw: mem::MaybeUninit<*mut libc::c_void> = mem::MaybeUninit::uninit();
unsafe {
 let size = NUM_PAGES * PAGE_SIZE;
 libc::posix_memalign(raw.as_mut_ptr(), PAGE_SIZE, size);
 libc::mprotect(*raw.as_mut_ptr(), size, libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
 std::ptr::write_bytes(*raw.as_mut_ptr(), 0xc3, size);  // for now, prepopulate with 'RET' calls
 contents = std::slice::from_raw_parts_mut(*raw.as_mut_ptr() as *mut u8, NUM_PAGES * PAGE_SIZE);
 raw.assume_init();
}

let mut mem = JitMemory {
 contents,
 offset: 0,
};

let mut jit = JitCompiler::new();
jit.jit_compile(&mut mem, prog, use_mbuff, update_data_ptr, helpers)?;
jit.resolve_jumps(&mut mem)?;

Ok(mem)
}
// https://github.com/qmonnet/rbpf/blob/7d2b019a51d48b7c91c269e5eec3084883a14b7c/src/jit.rs#L950