use std::io;
use std::io::Write; // <--- bring flush() into scope

fn main() {
    let v1 = vec![1, 2, 3];
    for item in v1 {
        print!("{} ", item);
        io::stdout().flush().unwrap();
    }

    println!("\n");

    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    for item in v2 {
        print!("{} ", item);
        io::stdout().flush().unwrap();
    }

    println!("");
}
