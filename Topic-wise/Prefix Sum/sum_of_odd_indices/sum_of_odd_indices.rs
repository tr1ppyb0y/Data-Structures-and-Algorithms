/*
    You are given an array and queries.
    Each query consists of two integers left and right.
    For every query, the task is to calculate the sum of all odd indices from left to right.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn odd_numbers_in_range(arr: Vec<i32>, queries: Vec<(usize, usize)>) -> Vec<i64> {
        let mut odd: Vec<i64> = vec![0; arr.len()];
        for (idx, val) in arr.iter().enumerate().skip(1) {
            odd[idx] = odd[idx - 1] + (val * (idx % 2) as i32) as i64;
        }

        let mut res: Vec<i64> = vec![0; queries.len()];
        for (idx, (l, r)) in queries.into_iter().enumerate() {
            if l == 0 {
                res[idx] = odd[r];
            } else {
                res[idx] = odd[r] - odd[l - 1];
            }
        }
        res
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

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let queries: Vec<(usize, usize)> = input
        .trim()
        .split_whitespace()
        .map(|x| {
            let mut range = x.split(',');
            let l = range.next().unwrap().parse().unwrap();
            let r = range.next().unwrap().parse().unwrap();
            (l, r)
        })
        .collect();

    let res = DSA::odd_numbers_in_range(arr.clone(), queries.clone());

    println!("Sum of numbers at odd indices -");
    println!(
        "Array: {:?}\nQueries: {:?}\nResult: {:?}",
        arr, queries, res
    );
}