fn main() {
    // Tuples can contain multiple data types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Access using `<tuple_name>.<index>`
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    // You can destructure a tuple into multiple variables
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    let product = x as f64 * y * z as f64;
    println!("{product}");
}
