/*
https://github.com/pola-rs/polars/blob/1f200d7762c94e438298ea7cb396c5eb2442005d/polars/polars-row/src/fixed.rs#L168
*/
pub(crate) unsafe fn encode_slice<T: FixedLengthEncoding>(
    input: &[T],
    out: &mut RowsEncoded,
    field: &SortField,
) {
    out.values.set_len(0);
    let values = out.values.spare_capacity_mut();
    for (offset, value) in out.offsets.iter_mut().skip(1).zip(input) {
        encode_value(value, offset, field.descending, values);
    }
}