/*
    Given an array, print sum of all sub-arrays.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn subarray_sum(arr: Vec<i64>) {
        for i in 0..arr.len() {
            for j in i..arr.len() {
                println!(
                    "{:?} - {}",
                    &arr[i..j + 1],
                    arr[i..j + 1].iter().sum::<i64>()
                );
            }
        }
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

    println!("Sub-arrays sum of array {:?} are,", arr);
    DSA::subarray_sum(arr.clone());
}
