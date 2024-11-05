/*
    Amazon, Meta.

    Given an array of integers arr and a value k, a subarray of an array is said to be good if it fulfills any one of the criteria:
        1. Length of the subarray is be even, and the sum of all the elements of the subarray must be less than k.
        2. Length of the subarray is be odd, and the sum of all the elements of the subarray must be greater than k.
        Your task is to find the count of good subarrays in arr.

    Note: 1 <= len(arr) <= 10^3
*/

use std::io;

struct DSA;

impl DSA {
    pub fn good_subarrays_easy(arr: Vec<i64>, k: i64) -> i64 {
        let mut count: i64 = 0;
        let mut summation: i64 = 0;
        let n: usize = arr.len();
        for idx in 0..n {
            for idy in idx..n {
                summation += arr[idy];
                count += ((summation < k) as i64) * (((idy - idx + 1) % 2 == 0) as i64);
                count += ((summation > k) as i64) * (((idy - idx + 1) % 2) as i64);
            }
            summation = 0;
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

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let k: i64 = input.trim().parse().expect("Not an integer");

    let res = DSA::good_subarrays_easy(arr.clone(), k);
    println!("Number of good sub-arrays are: {}", res);
}
