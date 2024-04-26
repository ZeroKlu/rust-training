fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    // This would be undefined behavior, since x is not defined
    // read(x);
    let x = true;
    read(x);
}
