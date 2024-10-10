/*
    A wire connects N light bulbs.
    Each bulb has a switch associated with it; however, due to faulty wiring,
    a switch also changes the state of all the bulbs to the right of the current bulb.
    Given an initial state of all bulbs, find the minimum number of switches
    you have to press to turn on all the bulbs. You can press the same switch multiple times.
    Note: 0 represents the bulb is off and 1 represents the bulb is on.
*/

use std::io;

struct DSA;

impl DSA {
    pub fn bulbs(arr: Vec<i64>) -> i64 {
        let mut count: i64 = (arr[0] == 0) as i64;
        let mut curr: i64 = arr[0];

        for state in arr.iter().skip(1) {
            if *state != curr {
                count += 1;
                curr = *state;
            }
        }
        count
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
        "{} number of flips is required to onn all the bulbs with initial configuration {:?}",
        DSA::bulbs(arr.clone()),
        arr
    );
}
