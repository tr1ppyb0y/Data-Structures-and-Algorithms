/*
    Asked in Adobe, Amazon, Hike.

    You are given an array of integers.
    Your task is to find the equilibrium index of the given array
    The equilibrium index of an array is an index such that,
    the sum of elements at lower indexes is equal to the sum of elements at higher indexes.
    If there are no elements at lower or higher,
    then the corresponding sum of elements is considered as 0.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn in_place_prefix_sum(arr: Vec<i64>) -> usize {
        let prefix: Vec<i64> = arr
            .iter()
            .scan(0, |acc, &x| {
                *acc = *acc + x;
                Some(*acc)
            })
            .collect();

        let suffix: Vec<i64> = arr
            .iter()
            .rev()
            .scan(0, |acc, &x| {
                *acc = *acc + x;
                Some(*acc)
            })
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .cloned()
            .collect();

        let mut curr = 0;
        if let Some(idx) = prefix
            .iter()
            .enumerate()
            .take(prefix.len() - 1)
            .find(|(idx, val)| {
                let match_ = curr == suffix[idx + 1];
                curr = **val;
                match_
            })
        {
            return idx.0;
        }

        return if curr != 0 {
            usize::MAX
        } else {
            return arr.len() - 1;
        };
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

    let res = DSA::in_place_prefix_sum(arr.clone());
    if res == usize::MAX {
        println!("There is no equilibrium index in array, {:?}", arr);
    } else {
        println!(
            "Equilibrium index of array {:?} is, {} [0-indexed].",
            arr, res
        );
    }
}
