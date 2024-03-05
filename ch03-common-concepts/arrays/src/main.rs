fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr[0]);

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
    ];
    println!("{}", months[1]);

    let preload = [0; 5];
    println!("{}", preload[4]);

    let mut empty = [' '; 5];
    empty[0] = 'a';
    println!("{}", empty[0]);
    println!("{}", empty[1]);
}
