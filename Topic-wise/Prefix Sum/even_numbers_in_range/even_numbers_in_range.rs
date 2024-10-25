/*
    You are given an array and queries with left and right indices.
    For every query, find the count of even numbers for every query in that range.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn even_numbers_in_range(arr: Vec<i64>, queries: Vec<(usize, usize)>) -> Vec<usize> {
        let acc: Vec<usize> = arr
            .iter()
            // slightly faster: map(|&x| (x % 2 == 0) as i32)
            .map(|x| if x % 2 == 0 { 1 as usize } else { 0 as usize })
            .scan(0, |acc, x| {
                *acc = *acc + x;
                Some(*acc)
            })
            .collect();

        let mut res: Vec<usize> = vec![0; queries.len()];
        for (idx, (l, r)) in queries.into_iter().enumerate() {
            if l == 0 {
                res[idx] = acc[r];
            } else {
                res[idx] = acc[r] - acc[l - 1];
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

    let arr: Vec<i64> = input
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

    let res = DSA::even_numbers_in_range(arr.clone(), queries.clone());

    println!(
        "Array: {:?}\nQueries: {:?}\nResult: {:?}",
        arr, queries, res
    );
}
