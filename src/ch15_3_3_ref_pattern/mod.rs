pub fn run() {
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // Destructure `mutable_tuple` to change the value of `first`.
        let (ref mut first, _) = mutable_tuple;
        // *first = Box::new(7u32);
        // first = 7u32;
        **first = 7u32;
    }

    println!("{:?}", mutable_tuple);

    // --

    let mut my_box = Box::new(7);
    *my_box = 8;
    println!("{:?}", my_box);
}
