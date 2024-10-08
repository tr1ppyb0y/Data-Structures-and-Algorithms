use std::io;

struct DSA;

impl DSA {
    pub fn count_elements(arr: Vec<i64>) -> i64 {
        let maximum = *arr.iter().max().unwrap();
        let mut count = 0;
        for num in arr {
            count += (num < maximum) as i64;
        }
        count
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();
    println!("{}", DSA::count_elements(arr));
}
