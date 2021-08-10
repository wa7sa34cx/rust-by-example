use std::path::Path;

// My own simple file read based database
#[derive(Debug)]
pub struct Db {
    file: &'static Path,
}

pub fn run() {
    let path = Path::new("hello.txt");

    let _db = Db { file: path };
}
