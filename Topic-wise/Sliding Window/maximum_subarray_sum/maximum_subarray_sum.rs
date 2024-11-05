/*
    Given an array arr, find the maximum subarray sum out of all possible subarray of length k.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn maximum_subarray_sum(arr: Vec<i64>, k: usize) -> i64 {
        let mut summation: i64 = arr[0..k].iter().sum();
        let mut res: i64 = summation;
        let n: usize = arr.len();
        for idx in k..n {
            summation = summation - arr[idx - k] + arr[idx];
            res = res.max(summation);
        }
        res
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

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let k: usize = input.trim().parse().expect("Not an integer");

    let res = DSA::maximum_subarray_sum(arr.clone(), k);
    println!(
        "Maximum subarray sum out of all possible subarray of length k is: {}",
        res
    );
}
