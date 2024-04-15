// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn get_median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    v[v.len() / 2]
}

fn get_mode(v: &mut Vec<i32>) -> i32 {
    let mut h = HashMap::new();
    for n in v.iter() {
        let count = h.entry(n).or_insert(0);
        *count += 1;
    }
    let mut c = 0;
    let mut n = 0;
    for (k, v) in &h {
        if v > &c {
            c = *v;
            n = **k;
        }
    }
    n
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 2, 2, 3, 4, 5, 6];
    println!("Numbers: {:?}", v);

    let mode = get_mode(&mut v);
    println!("Mode = {}", mode);

    let median = get_median(&mut v);
    println!("Median = {}", median);
}
