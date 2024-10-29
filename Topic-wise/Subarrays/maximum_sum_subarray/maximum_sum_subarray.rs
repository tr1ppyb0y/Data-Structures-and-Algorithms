/*
    Given an array, find the sub-array with maximum sum.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn maximum_sum_subarray(arr: Vec<i64>) -> (usize, usize, i64) {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut maximum = i64::MIN;
        for i in 0..arr.len() {
            let mut summation: i64 = arr[i];
            if summation > maximum {
                maximum = summation;
                x = i;
                y = i;
            }
            for j in i + 1..arr.len() {
                summation += arr[j];
                if summation > maximum {
                    maximum = summation;
                    x = i;
                    y = j;
                }
            }
        }
        (x, y, maximum)
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

    let (l, r, maximum) = DSA::maximum_sum_subarray(arr.clone());
    println!(
        "Sub-array {:?} with maximum sum of {} for array, {:?}.",
        &arr[l..r + 1],
        maximum,
        arr
    );
}
