#![allow(unused)]
#![feature(slice_as_chunks)]
fn main() {
    let slice: &[char] = &['l', 'o', 'r', 'e', 'm', '!'];

    let mut chunks = Vec::new();
    const N1:usize = 1;
    if N1!= 0 && slice.len()%N1 ==0{
        let mut iter = slice.chunks_exact(N1);
        let mut c = iter.next();
        while !c.is_none(){
            chunks.push(c.unwrap());
            c = iter.next();
        }
        assert_eq!(chunks, &[['l'], ['o'], ['r'], ['e'], ['m'], ['!']]);
    }

    let mut chunks = Vec::new();
    const N2:usize = 3;
    if N2!= 0 && slice.len()%N2 ==0{
        let mut iter = slice.chunks_exact(N2);
        let mut c = iter.next();
        while !c.is_none(){
            chunks.push(c.unwrap());
            c = iter.next();
        }
        assert_eq!(chunks, &[['l', 'o', 'r'], ['e', 'm', '!']]);
    }

}