// The sum of the squares of the first ten natural numbers is,
//              1² + 2² + ... + 10² = 385
//
// The square of the sum of the first ten natural numbers is,
//              (1 + 2 + ... + 10)² = 55² = 3025
//
// Hence the difference between the sum of the squares of the
//   first ten natural numbers and the square of the sum is
//              3025 - 385 = 2640
//
// Find the difference between the sum of the squares of the first
//   one hundred natural numbers and the square of the sum.

use thousands::Separable;

const MAX_NUM: usize = 100;

fn sum_of_squares(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let mut sum: usize = 0;
    for x in 1..=n {
        sum += x * x;
    }

    sum
}

fn square_of_sum(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let mut sum: usize = 0;
    for x in 1..=n {
        sum += x;
    }

    sum * sum
}

fn main() {
    let d = square_of_sum(MAX_NUM) - sum_of_squares(MAX_NUM);
    println!("{}", d.separate_with_commas());
}

// 25,164,150
