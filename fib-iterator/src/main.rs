struct Fibonacci {
    curr: u32,
    next: u32
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn main() {
    let mut fib = Fibonacci {
        curr: 1,
        next: 1,
    };

    for _ in 0..10 {
        println!("{:?}", fib.curr);
        fib.next();
    }
}
