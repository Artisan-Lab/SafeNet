/*
    From: https://github.com/slightlyoutofphase/staticvec/blob/a3557755b9ee29238e98302cfab550a75675f339/src/lib.rs#L2707
*/

  pub(crate) const fn bytes_to_data(values: &[u8]) -> MaybeUninit<[u8; N]> {
    // Get an uninitialized array of bytes, with `N` capacity.
    let mut res = MaybeUninit::uninit_array::<N>();
    // Move `values.len()` worth of bytes from `values` to `res`. I'm unaware of any other way that
    // this could be done currently that would leave us with something usable to create a StaticVec
    // for which the generic `N` could be *different* from `values.len()`, so thank
    // you, `const_loop`!
    let mut i = 0;
    while i < values.len() {
      // We've statically asserted that `values.len() <= N` before entering this overall function,
      // so there's no concern that we might go out of bounds here (although that would still just
      // result in compilation not actually succeeding at all due to the `const` index error).
      res[i] = MaybeUninit::new(values[i]);
      i += 1;
    }
    // Fill in any remaining "gaps", or do nothing if `values.len()` is the same as `N`.
    let mut i = values.len();
    while i < N {
      res[i] = MaybeUninit::zeroed();
      i += 1;
    }
    // Convert `res` from an instance of `[MaybeUninit<u8>; N]` to one of `[u8; N]`, and then return
    // it as an instance of `MaybeUninit<[u8; N]>` that can be used to construct a `StaticVec`.
    MaybeUninit::new(unsafe { MaybeUninit::array_assume_init(res) })
  }