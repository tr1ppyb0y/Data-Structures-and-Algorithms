use std::io;

struct DSA;

impl DSA {
    pub fn perfect_numbers(n: usize) -> bool {
        let mut result = Vec::new();
        for i in 1..n {
            if n % i == 0 {
                result.push(i);
            }
        }
        result.iter().sum::<usize>() == n
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    println!("Is {} perfect number? {}", n, DSA::perfect_numbers(n));
}
