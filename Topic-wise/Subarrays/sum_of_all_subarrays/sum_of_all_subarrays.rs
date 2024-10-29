/*
    Given an array, find the sum of all sub-arrays.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn sum_of_all_subarrays(arr: Vec<i64>) -> i64 {
        let mut summation: i64 = 0;
        let n: usize = arr.len();
        for (idx, val) in arr.into_iter().enumerate() {
            summation += ((idx + 1) as i64 * (n - idx) as i64 * val) as i64;
        }
        summation
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

    let summation = DSA::sum_of_all_subarrays(arr.clone());
    println!("Sum of all subarrays: {}", summation);
}
