extern crate itertools;

struct DecimalDigitsOf {
    val: u64,
}

impl Iterator for DecimalDigitsOf {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.val == 0 { None }
        else { 
            let digit = (self.val % 10) as u8;
            self.val /= 10;
            Some(digit)
        }
    }
}

fn decimal_digits(val: u64) -> Vec<u8> {
    let mut digits: Vec<_> = DecimalDigitsOf { val: val }.collect();
    digits.reverse();
    digits
}

fn vec_is_palindrome<A: PartialEq>(vec: Vec<A>) -> bool {
    let len = vec.len();
    (0 .. len / 2).all(|i| { vec[i] == vec[len - 1 - i] })
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

