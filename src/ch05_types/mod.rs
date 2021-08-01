#![allow(overflowing_literals)]

pub fn run() {
    let decimal = 65.4321_f32;

    let integer = decimal as u8;
    println!("integer: {}", integer);

    let mychar = integer as char;
    println!("char: {}", mychar);

    let mychar = 'a';
    println!("char: {}", mychar as u8);

    // alphabet as u8
    let alpha = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    for item in alpha {
        print!("{} ", item as u8);
    }

   
    // generate alphabet
    let mut alpha: Vec<char> = Vec::new();

    for i in 0..26 {
        alpha.push((b'a' + i) as char);
    }
    println!("\n\nalphabet:\n {:?}", alpha);


    // generate aplhabet 2
    let alpha = (0..26).map(|i| (b'a' + i) as char).collect::<Vec<char>>();
    println!("\n\nalphabet2:\n {:?}", alpha);


    //------
    println!("\n\n");

    println!("1000 as a u8 is : {}", (1000) as u8);
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
    println!("1000 mod 256 is : {}", 1000 % 256);

    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is : {}", 128 as i8);

    //-----
    let x = 65;
    println!("\n\n{}", x as u8 as char);

    //------
    println!("\n\n");

    // 300.0 is 255
    println!("300.0 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);

    //------
    println!("\n\n");

    let k = 255u8;
    let i = 1;
    let f = 1.0;
    let h = 100u64;
    println!("size of `k` in bytes: {}", std::mem::size_of_val(&k));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    println!("size of `h` in bytes: {}", std::mem::size_of_val(&h));

}
