// 2520 is the smallest number that can be divided by each of the numbers from 
// 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the
// numbers from 1 to 20?

use thousands::Separable;

const MAX_NUM: usize = 20;

fn is_factor_of(f: usize, n: usize) -> bool {
    if f == 0 {
        println!("Zero cannot be a factor!");
        return false;
    }
    n % f == 0
}

// Try to improve this - very brute-force
fn lowest_multiple_of_factors_below(n: usize) -> usize {
    if n == 0 {return 0;}

    let mut mult: usize = 0;

    loop {
        let mut found = true;
        mult += n;
        for i in 1..=n {
            if !is_factor_of(i, mult) {
                found = false;
                break;
            }
        }
        if found {break;}
    }

    mult
}

fn main() {
    let m: usize = lowest_multiple_of_factors_below(MAX_NUM);
    println!("{}", m.separate_with_commas());
}

// 232,792,560
