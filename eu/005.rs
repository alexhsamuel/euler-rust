use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

type Factors = HashMap<u64, u8>;

/*
 * Factorizes `n`, and returns a map from prime factor to multiplicity.
 */
fn factor(n: u64) -> Factors {
    let mut n = n;
    let mut factors: Factors = HashMap::new();
    for i in 2 .. n + 1 {
        if n % i == 0 {
            let mut c = 0;
            while n % i == 0 {
                c += 1;
                n /= i;
            }
            factors.insert(i, c);
            if n == 1 { break; }
        }
    }
    factors
}

/*
 * For two factorizations, computes, the LCM factorization.
 */
fn factor_lcm(fac0: &Factors, fac1: &Factors) -> Factors {
    // Find the set of common prime factors.
    fac0.keys().chain(fac1.keys())
        .map(|&f| { f })
        .collect::<HashSet<_>>()
        .into_iter()
    // For each factor, use the larger multiplicity.
        .map(|f| { 
            let c0 = fac0.get(&f).map_or(0, |&c| { c });
            let c1 = fac1.get(&f).map_or(0, |&c| { c });
            (f, cmp::max(c0, c1))
        })
        .collect()
}

fn main() {
    let factors = 
        (1 .. 20)
        .fold(Factors::new(), |f, n| { factor_lcm(&f, &factor(n)) });
    let result = 
        factors.iter()
        .fold(1u64, |r, (&f, &c)| { r * f.pow(c as u32) });
    println!("{}", result);
}

