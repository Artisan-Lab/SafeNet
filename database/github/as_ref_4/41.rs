pub fn crate_description(crates: &[impl AsRef<str>]) -> String {
    let mut descr = String::from(" {");
    descr.push_str(crates[0].as_ref());
}
/*
https://github.com/glaubitz/rust/blob/1f5768bc67ecb025342770e14e03699699965706/src/bootstrap/builder.rs#L125
*/
