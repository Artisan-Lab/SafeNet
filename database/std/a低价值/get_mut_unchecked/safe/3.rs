use std::sync::Arc;

let x: Arc<str> = Arc::from("Hello, world!");
let y: Arc<str> = x.clone(); // Clone x to obtain another Arc<str>
let y_bytes: Arc<[u8]> = y.as_bytes().into(); // Convert the Arc<str> to an Arc<[u8]>
let y_mut: &mut [u8] = Arc::make_mut(&mut y_bytes); // Get a mutable reference to the contents
y_mut.fill(0xff); // Fill the slice with 0xff bytes
println!("{}", &*x); // "Hello, world!"
