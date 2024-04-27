// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143

use thousands::Separable;

const N: u64 = 600_851_475_143;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n %2 == 0 {
        return false;
    } 
    for d in (3 .. (n as f64).sqrt() as u64 + 1).step_by(2) {
        if n % d == 0 {
            return false;
        }
    }
    return true;
}

fn prime_factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    if is_prime(n) {
        factors.push(n);
        return factors;
    }
    for i in (3 .. (n as f64).sqrt() as u64 + 1).step_by(2) {
        if n % i != 0 {
            continue;
        }
        if is_prime(i) {
            factors.push(i);
        }
    }
    factors
}

fn main() {
    let f = prime_factors(N);
    println!("{}", f.iter().max().unwrap().separate_with_commas());
}

// 6,857
