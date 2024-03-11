fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    for n in slice.iter() {
        println!("{n}");
    }
    assert_eq!(slice, &[2, 3]);
}
