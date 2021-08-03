mod abc;

use abc::Abc;

pub fn run() {
    // contents is private!
    // let abc = abc::Abc { contents: "info" };

    // let abc = abc::Abc::new("info");
    let abc = Abc::new("info");

    println!("{:#?}", abc);
}