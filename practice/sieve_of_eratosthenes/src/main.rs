const MAX_VALUE: usize = 1000;

fn sieve(n: usize) -> Vec<bool> {
    let mut prime_array = vec![true; n + 1];
    prime_array[0] = false;
    prime_array[1] = false;

    for i in 2 .. n + 1 {
        if prime_array[i] {
            for j in (2 * i .. n + 1).step_by(i) {
                prime_array[j] = false;
            }
        }
    }

    prime_array
}

fn main() {
    let prime_array = sieve(MAX_VALUE);
    if prime_array.len() < 1 {
        return;
    }
    println!("Primes up to {MAX_VALUE}");
    println!("------------------------------------------------------------------------------------");
    let mut primes = String::new();
    for i in 0 .. prime_array.len() {
        if !prime_array[i] {
            continue;
        }
        primes.push_str(&(format!("{:>6}", i)));
        if primes.len() > 80 {
            println!("{}", primes);
            primes = String::new();
        }
    }
    if primes.len() > 0 {
        println!("{}", primes);
    }
}
