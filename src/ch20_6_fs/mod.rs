use std::error::Error;
use std::fs::File;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut _f = File::open("hello.txt")?;

    Ok(())
}
