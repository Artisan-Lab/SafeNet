use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut f = File::create("foo.txt")?;
    f.set_len(10)?;
    // 无需unsafe
    Ok(())
}