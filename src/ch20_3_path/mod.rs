use std::path::Path;

pub fn run() {
    let path = Path::new("/etc");
    println!("{}", path.display());

    let new_path = path.join("hosts");
    println!("{}", new_path.display());

    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}