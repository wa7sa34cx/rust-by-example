struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr + self.next;
        self.curr = self.next;
        self.next = next;

        Some(self.curr)
    }
}

pub fn run() {
    let fibonacci = Fibonacci { curr: 0, next: 1 };

    for i in fibonacci.take(10) {
        println!("{}", i);
    }
}
