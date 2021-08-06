macro_rules! create_function {
    ($fn_name:ident) => {
        fn $fn_name() {
            println!("You called {:?}()", stringify!($fn_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

pub fn run() {
    foo();
    bar();

    print_result!(1u32 + 1);

    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });


}