/*
    Say you have an array, for which the ith element is the price of a given stock on day i.
    If you were only permitted to complete at most one transaction (ie, buy one and sell one share of the stock),
    design an algorithm to find the maximum profit. Assume trading happend atleast for one day.
    Return the maximum possible profit.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn one_stock(arr: Vec<i64>) -> i64 {
        let mut buy: i64 = arr[0];
        let mut profit: i64 = 0;
        for price in arr.iter().skip(1) {
            buy = std::cmp::min(buy, *price);
            profit = std::cmp::max(profit, *price - buy);
        }
        profit
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
        "{} is the maximum profit that can be made.",
        DSA::one_stock(arr.clone())
    );
}
