#[derive(Debug)]
pub struct Abc<T> {
    contents: T,
}

impl<T> Abc<T> {
    pub fn new(contents: T) -> Abc<T> {
        Abc {
            contents,
        }
    }
}