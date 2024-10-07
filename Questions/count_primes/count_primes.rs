use std::io;

struct DSA;

impl DSA {
    pub fn count_primes(n: usize) -> usize {
        let mut primes = vec![true; n + 1];
        primes[0] = false;
        primes[1] = false;
        for i in 2..n {
            if primes[i] {
                for j in (i * i..=n).step_by(i) {
                    primes[j] = false;
                }
            }
        }
        primes.iter().filter(|&b| *b == true).count()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    println!(
        "There are {} primes less than or equal to {}",
        DSA::count_primes(n),
        n
    );
}
