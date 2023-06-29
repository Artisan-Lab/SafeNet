#![allow(unused)]
#![feature(slice_as_chunks)]
fn main() {
    let slice: &mut [char] = &mut ['l', 'o', 'r', 'e', 'm', '!'];

    let mut chunks = Vec::new();
    const N1:usize = 1;
    if N1!= 0 && slice.len()%N1 ==0{
        let mut iter = slice.chunks_exact(N1);
        let mut c = iter.next();
        while !c.is_none(){
            chunks.push(c.unwrap());
            c = iter.next();
        }
        chunks[0] = &['L'];

        assert_eq!(chunks, &[['L'], ['o'], ['r'], ['e'], ['m'], ['!']]);
    }
    slice[0] = 'L';

    let mut chunks = Vec::new();
    const N2:usize = 3;
    if N2!= 0 && slice.len()%N2 ==0{
        let mut iter = slice.chunks_exact(N2);
        let mut c = iter.next();
        while !c.is_none(){
            chunks.push(c.unwrap());
            c = iter.next();
        }
        chunks[1] = &['a', 'x', '?'];
        assert_eq!(chunks, &[['L', 'o', 'r'], ['a', 'x', '?']]);
    }
    slice[3]='a';
    slice[4]='x';
    slice[5]='?';
    assert_eq!(slice, &['L', 'o', 'r', 'a', 'x', '?']);

}