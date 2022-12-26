use std::fs::File;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
    let mut fd = File::create("FirstFile.txt")?;
    fd.write_all(b"Hello World")?;
    Ok(())
}

