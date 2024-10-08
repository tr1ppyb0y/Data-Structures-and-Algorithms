/*
    Given an integer array containing n distinct integers,
    you have to find all the leaders in array.
    An element is a leader if it is strictly greater than all the elements to its right side.
    NOTE: The rightmost element is always a leader.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn leaders_in_array(arr: Vec<i64>) -> Vec<i64> {
        let mut leaders: Vec<i64> = vec![arr.last().unwrap().clone()];
        for val in arr.iter().rev().skip(1) {
            if val > leaders.last().unwrap() {
                leaders.push(*val);
            }
        }
        leaders
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
    println!(
        "{:?} are the leaders in array {:?}",
        DSA::leaders_in_array(arr.clone()),
        arr
    );
}
