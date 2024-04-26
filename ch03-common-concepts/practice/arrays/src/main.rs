fn main() {
    // Declare as `[<type>; <num_elem>]`
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Access using `<arr_name>[<index>]`
    println!("{}", arr[0]);

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
    ];
    println!("{}", months[1]);

    // We can prepopulate using `[<value>; <num_elem>]`
    let preload = [0; 5];
    println!("{}", preload[4]);

    // Values can be mutable
    let mut empty = [' '; 5];
    empty[0] = 'a';
    println!("{}", empty[0]);
    println!("{}", empty[1]);

    // This would result in an 'index out of bounds' error.
    // let invalid = empty[6];
}
