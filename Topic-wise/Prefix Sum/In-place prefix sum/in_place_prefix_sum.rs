/*
    Given an array A. Construct prefix sum of the array in the given array itself.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn in_place_prefix_sum(arr: &mut Vec<i64>) -> &Vec<i64> {
        let n = arr.len();
        for i in 1..n {
            arr[i] = arr[i - 1] + arr[i];
        }
        arr
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();

    let orignal = arr.clone();
    println!(
        "{:?} is the prefix of array, {:?}",
        DSA::in_place_prefix_sum(&mut arr),
        orignal
    );
}
