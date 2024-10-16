/*
    Asked in Google.

    You are given an integer array.
    You have to perform k operations.
    In one operation, you can remove either the leftmost or the rightmost element of the array.
    Find and return the maximum possible sum of the k elements that were removed after the k operations.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn pick_from_both_side(arr: Vec<i64>, k: usize) -> i64 {
        let n = arr.len();
        let mut res: i64 = arr[n - k..n].iter().sum();
        let mut curr: i64 = res;
        for i in 0..k {
            curr = curr - arr[n - k + i] + arr[i];
            res = res.max(curr);
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

    println!(
        "{} is the maximum sum.",
        DSA::pick_from_both_side(arr.clone(), k),
        k,
        arr
    );
}
