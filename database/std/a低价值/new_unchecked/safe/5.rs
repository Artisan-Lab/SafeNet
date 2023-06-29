use std::num::NonZeroU16;

let x = NonZeroU16::new(42).unwrap();
println!("{}", x.get());
