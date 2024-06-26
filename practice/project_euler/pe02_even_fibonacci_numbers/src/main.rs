// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 
//   1 and 2, the first 10 terms will be:
//                     1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
//   find the sum of the even-valued terms.

use std::collections::HashMap;
use thousands::Separable;

const MAX_NUM: usize = 4_000_000;

fn fibonacci(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    match cache.get(&n) {
        Some(num) => {
            *num
        }
        None => {
            let f = fibonacci(n-1, cache) + fibonacci(n-2, cache);
            cache.insert(n, f);
            f
        }
    }
}

fn default_hash() -> HashMap<usize, usize> {
    let mut h = HashMap::new();
    h.insert(0, 0);
    h.insert(1, 1);
    h
}

fn main() {
    let mut n: usize = 0;
    let mut sum: usize = 0;
    let mut f: usize = 0;
    let mut hash = default_hash();
    loop {
        if f > MAX_NUM {
            break;
        }
        if f % 2 == 0 {
            sum += f;
        }
        n += 1;
        f = fibonacci(n, &mut hash);
    }
    println!("{}", sum.separate_with_commas());
}

// 4,613,732