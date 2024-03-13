struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    // You can destructure a tuple struct just like a tuple
    let Color(r, g, b) = black;
    println!("{r}, {g}, {b}");

    let origin = Point(0, 0, 0);
    // You can use tuple-style dot syntax to access elements
    let x = origin.0;
    println!("{x}");
}