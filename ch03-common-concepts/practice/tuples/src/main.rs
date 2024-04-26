fn main() {
    // Can contain multiple data types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Access using `<tuple_name>.<index>`
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    // Can deconstruct
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");
}
