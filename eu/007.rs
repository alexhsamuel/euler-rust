struct Primes {
    primes: Vec<u64>,
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match self.primes.last() {
            None => {
                self.primes.push(2);
                Some(2u64)
            }
            Some(&n) => {
                let mut n = n;
                loop {
                    n += 1;
                    if self.primes.iter().all(|p| { n % p != 0 }) {
                        self.primes.push(n);
                        return Some(n);
                    }
                }
            }
        }
    }

}

fn primes() -> Primes {
    Primes { primes: Vec::new() }
}

fn main() {
    println!("{}", primes().nth(10000).unwrap());
}

