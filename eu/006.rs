fn sum<I: Iterator<Item=u64>>(values: I) -> u64 {
    values.fold(0, |s, x| { s + x })
}

fn main() {
    let n = 100u64;
    let sum1 = sum(1u64 .. n + 1);
    let sum2 = sum((1u64 .. n + 1).map(|n| { n * n }));
    println!("{}", sum1 * sum1 - sum2);
}

