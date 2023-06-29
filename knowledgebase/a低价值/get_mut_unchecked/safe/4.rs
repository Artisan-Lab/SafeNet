use std::sync::Arc;

let x: Arc<&str> = Arc::new("Hello, world!");

// create a new Arc with the same value as x
let y = x.clone();

// create a new string with the desired value
let s = String::from("Oh, no!");

// create a new Arc pointing to the string
let z: Arc<&str> = Arc::from(&s[..]);

// replace the value of y with z
drop(Arc::try_unwrap(y).unwrap_or_else(|_| panic!("Arc is not unique")))?;
let y = z;

println!("{}", &*x); // This will print "Hello, world!"
