// If we list all the natural numbers below 10 that are multiples of 3 or 5,
// we get 3, 5, 6, and 9. The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.

const MAX_NUM: usize = 1000;

fn sum_multiples(n: usize) -> usize {
    let mut sum: usize = 0;
    for i in 3 .. n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

fn main() {
    println!("{}", sum_multiples(MAX_NUM));
}

// 233168
