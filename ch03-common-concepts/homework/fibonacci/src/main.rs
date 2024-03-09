use std::collections::HashMap;
use thousands::Separable;

fn main() {
    let mut n = -1;
    while n < 0 {
        n = get_int("Please enter a non-negative integer:");
    }
    let f = fib(n, &mut default_hash());
    println!("F({}) = {}\n", n, f.separate_with_commas());
}

// fn fib_naive(n: i64) -> i64 {
//     if n < 2 {n} else {fib_naive(n-1) + fib_naive(n-2)}
// }

fn fib(n: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    match cache.get(&n) {
        Some(num) => {
            *num
        }
        None => {
            let f = fib(n-1, cache) + fib(n-2, cache);
            cache.insert(n, f);
            f
        }
    }
}

fn default_hash() -> HashMap<i64, i64> {
    let mut h = HashMap::new();
    h.insert(0, 0);
    h.insert(1, 1);
    h
}

fn get_int(message: &str) -> i64 {
    println!("\n{}", message);
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    match line.trim().parse::<i64>() {
        Ok(num) => if num > -1 {num} else {-1},
        Err(_) => -1
    }
}