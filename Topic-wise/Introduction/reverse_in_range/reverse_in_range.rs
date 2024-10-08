use std::io;

struct DSA;

impl DSA {
    pub fn reverse_in_range(arr: Vec<i64>, a: usize, b: usize) -> Vec<i64> {
        let mut result = arr[..a].to_vec();
        result.extend(arr[a..=b].iter().rev());
        result.extend(arr[b + 1..].to_vec());
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

    println!("Enter the range: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let a: usize = input
        .split_whitespace()
        .next()
        .expect("Invalid input")
        .parse()
        .expect("Invalid input");

    let b: usize = input
        .split_whitespace()
        .last()
        .expect("Invalid input")
        .parse()
        .expect("Invalid input");

    println!(
        "Reversed {:?} in between {}, {} [0-indexed]: {:?}",
        arr,
        a,
        b,
        DSA::reverse_in_range(arr.clone(), a, b)
    );
}
