use std::env;
use std::io::Write;

fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let limit = (n as f64).sqrt() as u64;
    let mut result: Vec<u64> = Vec::new();
    for i in 2 .. limit + 1 {
        while n % i == 0 {
            result.push(i);
            n = n / i;
        }
    }
    result
}

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn usage() -> ! {
    let prog = env::args().next().unwrap();
    println_stderr!("usage: {} VAL", prog);
    std::process::exit(2);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 { usage(); }
    let arg = match args[1].parse::<u64>() {
        Ok(a) => a,
        Err(_) => usage(),
    };

    println!("{:?}", factors(arg));
}

