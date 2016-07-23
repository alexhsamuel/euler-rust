struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<<Fibonacci as Iterator>::Item> {
        let next = self.curr + self.next;
        self.curr = self.next;
        self.next = next;
        Some(self.curr)
    }

}

fn main() {
    let fib = Fibonacci { curr: 1, next: 2 };
    let result = fib
        .take_while(|&n| n <= 4000000u64)
        .filter(|&n| n % 2 == 0)
        .fold(0, std::ops::Add::add);
    println!("{}", result);
}
