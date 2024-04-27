// A palindromic number reads the same both ways. The largest palindrome made from the product of two 
// 2-digit numbers is 9009 = 91 * 99.
// 
// Find the largest palindrome made from the product of two 3-digit numbers.

use thousands::Separable;

const MAX: i32 = 999;
const MIN: i32 = 100;

fn is_palindrome(word: String) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let mut i = 0;
    while i < chars.len() / 2 {
        if chars[i] != chars[chars.len() - 1 - i] {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let mut prod = 0;
    for p in (MIN..=MAX).rev() {
        for q in (MIN..=MAX).rev() {
            let x = p * q;
            if x > prod && is_palindrome(x.to_string()) {
                prod = x;
            }
        }
    }
    println!("{}", prod.separate_with_commas());
}

// 906,609
