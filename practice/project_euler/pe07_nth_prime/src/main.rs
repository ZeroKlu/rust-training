// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
//   we can see that the 6th prime is 13.
//
// What is the 10001st prime number?

use thousands::Separable;

const MAX_NUM: usize = 10001;

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n %2 == 0 {
        return false;
    } 
    for d in (3 .. (n as f64).sqrt() as usize + 1).step_by(2) {
        if n % d == 0 {
            return false;
        }
    }
    return true;
}

fn nth_prime(n: usize) -> usize {
    let mut c: usize = 1;
    let mut p: usize = 2;
    let mut s: usize = 3;
    while c < n {
        if is_prime(s) {
            p = s;
            c += 1;
        }
        s += 2;
    }
    p
}

fn main() {
    println!("{}", nth_prime(MAX_NUM).separate_with_commas())
}

// 104,743