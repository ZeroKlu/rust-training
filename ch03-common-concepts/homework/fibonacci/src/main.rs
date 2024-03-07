fn main() {
    let f: i32 = fib(4);
    println!("{}", f);
}

fn fib(n: i32) -> i32 {
    if n < 2 {n} else {fib(n-1) + fib(n-2)}
}

// I'll come back and add more algorithms later
