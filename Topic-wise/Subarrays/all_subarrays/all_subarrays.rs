/*
    Given an array, print all sub-arrays.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn all_subarrays(arr: Vec<i32>) {
        for i in 0..arr.len() {
            for j in i..arr.len() {
                println!("{:?}", &arr[i..j + 1]);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();

    println!("All sub-arrays of array {:?} are,", arr);
    DSA::all_subarrays(arr.clone());
}
