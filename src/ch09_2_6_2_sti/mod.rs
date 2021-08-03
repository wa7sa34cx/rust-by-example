pub fn run() {
    let vec = vec![1, 2, 3];
    println!("Find 2 in vec1: {:?}", vec.iter().find(|&&x| x > 2));
    println!("Find 2 in vec1: {:?}", vec.iter().position(|x| x % 2 == 0));
}
