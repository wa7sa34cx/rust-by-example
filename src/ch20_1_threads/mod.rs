use std::thread;

const NTHREADS: u32 = 10;

pub fn run() {
    let mut children = vec![];

    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("this is a thread number {}", i);
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
