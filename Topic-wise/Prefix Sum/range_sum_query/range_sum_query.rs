/*
    You are given an integer array arr.
    You are also given a 2D integer array range with dimensions n x 2, where each row denotes a [l, r] query.
    For each query, you have to find the sum of all elements from l to r indices in arr (0 - indexed).
    More formally, find arr[l] + arr[l + 1] + arr[l + 2] +... + arr[r - 1] + arr[r] for each query.
*/

use std::io;

struct DSA {
    arr: Vec<i64>,
    interval: Vec<(usize, usize)>,
}

impl DSA {
    fn new(arr: Vec<i64>, interval: Vec<(usize, usize)>) -> Self {
        DSA { arr, interval }
    }

    fn in_place_prefix_sum(&mut self) {
        for idx in 1..self.arr.len() {
            self.arr[idx] += self.arr[idx - 1];
        }
    }

    fn range_sum_query(&mut self) -> Vec<i64> {
        let mut res = vec![0, self.interval.len() as i64];
        self.in_place_prefix_sum();

        for (idx, &(l, r)) in self.interval.iter().enumerate() {
            if l == 0 {
                res[idx] = self.arr[r];
            } else {
                res[idx] = self.arr[r] - self.arr[l - 1];
            }
        }

        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let interval: Vec<(usize, usize)> = input
        .trim()
        .split_whitespace()
        .map(|x| {
            let mut range = x.split(',');
            let l: usize = range.next().unwrap().parse().expect("Parse error in l.");
            let r: usize = range.next().unwrap().parse().expect("Parse error in r.");
            (l, r)
        })
        .collect();

    let mut dsa = DSA::new(arr, interval);
    let res = dsa.range_sum_query();

    println!(
        "Range sum - {:?}\nArray - {:?}\nRanges - {:?}",
        res, dsa.arr, dsa.interval
    )
}
