fn next_chunk<T: Copy, const N: usize>(
    it: &mut impl Iterator<Item = T>,
) -> Result<[T; N], IntoIter<T, N>> {
    let mut buffer = [None; N];
    let mut i = 0;
    while i < N {
        match it.next() {
            Some(x) => {
                buffer[i] = Some(x);
                i += 1;
            }
            None => {
                return Err(IntoIter::new(buffer.iter().flatten().cloned()));
            }
        }
    }

    let array = std::array::from_iter(buffer.iter().map(|x| x.unwrap()));
    Ok(array)
}

let r: [_; 4] = next_chunk(&mut (10..16)).unwrap();
assert_eq!(r, [10, 11, 12, 13]);
let r: IntoIter<_, 40> = next_chunk(&mut (10..16)).unwrap_err();
assert_eq!(r.collect::<Vec<_>>(), vec![10, 11, 12, 13, 14, 15]);
