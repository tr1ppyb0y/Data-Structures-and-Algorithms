use std::io;

struct DSA;

impl DSA {
    pub fn reverse_the_array(mut arr: Vec<i64>) -> Vec<i64> {
        let n: usize = arr.len();
        for i in 0..n / 2 {
            arr.swap(i, n - i - 1);
        }
        arr
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

    println!(
        "Reversed {:?}: {:?}",
        arr.clone(),
        DSA::reverse_the_array(arr)
    );
}
