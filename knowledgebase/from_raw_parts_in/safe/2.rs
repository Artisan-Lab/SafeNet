let mut v = Vec::with_capacity(3);
v.push(1);
v.push(2);
v.push(3);

let mut v = std::mem::ManuallyDrop::new(v);
let p = v.as_mut_ptr();
let len = v.len();
let cap = v.capacity();
unsafe {
    for i in 0..len {
        std::ptr::write(p.add(i), 4 + i);
    }

    let rebuilt = Vec::from_raw_parts(p, len, cap);
    assert_eq!(rebuilt, [4, 5, 6]);
}
