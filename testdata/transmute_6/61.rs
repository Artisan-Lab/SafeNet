pub fn cmov_i64(condition: bool, src: &i64, dest: &mut i64) {
    let src_transmuted = unsafe { core::mem::transmute::<&i64, &u64>(src) };
    let dest_transmuted = unsafe { core::mem::transmute::<&mut i64, &mut u64>(dest) };

    cmov_u64(condition, src_transmuted, dest_transmuted);
}
/*
https://github.com/frankfanslc/mc-oblivious/blob/456bf3612d7455520c13520d0ffd3916547a88a4/aligned-cmov/src/cmov_impl_asm.rs#L76
*/