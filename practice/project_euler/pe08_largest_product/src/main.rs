use std::env;
use std::fs;
use thousands::Separable;

fn main() {
    let path = format!("{}\\..\\data\\008_1000_digits.txt", get_working_dir());
    let big_number = fs::read_to_string(path)
        .expect("Failed to read file!")
        .replace("\r\n", "");
    let mut prod: u64 = 0;
    let mut num = "";
    for i in 0..big_number.len() - 12 {
        let sm_num = &big_number[i..i + 13];
        let mut p: u64 = 1;
        for elem in sm_num.chars() {
            let n: u64 = elem.to_digit(10).unwrap().into();
            p *= n;
        }
        if p > prod {
            prod = p;
            num = sm_num;
        }
    }
    println!("{} -> {}", num.separate_with_commas(), prod.separate_with_commas());
}

fn get_working_dir() -> String {
    let path = env::current_dir();
    match path {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}
