use std::io;

struct DSA;

impl DSA {
    pub fn rotate_the_array(arr: Vec<i64>, mut b: usize) -> Vec<i64> {
        let n = arr.len();
        b %= n;
        b = n - b;
        let mut result = arr[b..].to_vec();
        result.extend(arr[..b].to_vec());
        result
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let arr: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let b: usize = input.trim().parse().expect("Invalid input");

    println!(
        "Right rotated array of {:?} by {}: {:?}",
        arr.clone(),
        b,
        DSA::rotate_the_array(arr, b)
    );
}
