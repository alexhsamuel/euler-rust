extern crate itertools;

fn decimal_digits(val: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut val = val;
    while val > 0 {
        digits.push((val % 10) as u8);
        val /= 10;
    }
    digits.reverse();
    digits
}

fn vec_is_palindrome<A: PartialEq>(vec: Vec<A>) -> bool {
    let len = vec.len();
    for i in 0 .. len / 2 {
        if vec[i] != vec[len - 1 - i] {
            return false;
        }
    }
    true
}

fn is_decimal_palindrome(val: u64) -> bool {
    vec_is_palindrome(decimal_digits(val))
}

fn main() {
    let mut products: Vec<_> = 
        itertools::Product::new(100 .. 1000, 100 .. 1000)
        .map(|(a, b)| { (a * b) as u64 })
        .filter(|&n| { is_decimal_palindrome(n) })
        .collect();
    products.sort();
    println!("{:?}", products);
}

