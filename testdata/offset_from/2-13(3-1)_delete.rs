/// Read `u8`s and returns a byte slice up until a given predicate returns true
/// * `ctx` - The context required by `u8`. It will be passed to every `u8` when constructing.
/// * `predicate` - the predicate that decides when to stop reading `u8`s
/// The predicate takes two parameters: the number of bits that have been read so far,
/// and a borrow of the latest value to have been read. It should return `true` if reading
/// should now stop, and `false` otherwise
fn read_slice_with_predicate<'a, Ctx: Copy, Predicate: FnMut(usize, &u8) -> bool>(
    input: &'a BitSlice<u8, Msb0>,
    ctx: Ctx,
    mut predicate: Predicate,
) -> Result<(&'a BitSlice<u8, Msb0>, &[u8]), DekuError>
    where
        u8: DekuRead<'a, Ctx>,
{
    let mut rest = input;
    let mut value;

    loop {
        let (new_rest, val) = u8::read(rest, ctx)?;
        rest = new_rest;

        let read_idx = unsafe { rest.as_bitptr().offset_from(input.as_bitptr()) } as usize;
        value = input[..read_idx].domain().region().unwrap().1;

        if predicate(read_idx, &val) {
            break;
        }
    }

    Ok((rest, value))
}
//https://github.com/sharksforarms/deku/blob/d04559605bed4476b1a78aaffd843377d6ab8fe6/src/impls/slice.rs#L28