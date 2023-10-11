fn conflicts(&self, other: &Self) -> bool {
    debug_assert!(self.range.0 <= self.range.1);
    debug_assert!(other.range.0 <= other.range.1);

    if other.range.0 >= self.range.1 || self.range.0 >= other.range.1 {
        return false;
    }

    // The Diophantine equation which describes whether any integers can combine the data pointers and strides of the two arrays s.t.
    // they yield the same element has a solution if and only if the GCD of all strides divides the difference of the data pointers.
    //
    // That solution could be out of bounds which mean that this is still an over-approximation.
    // It appears sufficient to handle typical cases like the color channels of an image,
    // but fails when slicing an array with a step size that does not divide the dimension along that axis.
    //
    // https://users.rust-lang.org/t/math-for-borrow-checking-numpy-arrays/73303
    let ptr_diff = unsafe { self.data_ptr.offset_from(other.data_ptr).abs() };
    let gcd_strides = gcd(self.gcd_strides, other.gcd_strides);

    if ptr_diff % gcd_strides != 0 {
        return false;
    }

    // By default, a conflict is assumed as it is the safe choice without actually solving the aliasing equation.
    true
}