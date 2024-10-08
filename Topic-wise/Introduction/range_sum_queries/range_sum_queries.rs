use std::io;

struct DSA;

impl DSA {
    pub fn range_sum_queries(
        m: usize,
        n: usize,
        arr: Vec<i32>,
        queries: Vec<(i32, i32)>,
    ) -> Vec<i32> {
        let acc: Vec<i32> = arr
            .iter()
            .scan(0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect();

        let mut res = vec![0; queries.len()];
        for i in 0..n {
            if queries[i as usize].0 == 0 {
                res[i] += acc[queries[i].1 as usize];
            } else {
                res[i] +=
                    acc[queries[i as usize].1 as usize] - acc[(queries[i as usize].0 - 1) as usize];
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
    let m: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let queries: Vec<(i32, i32)> = input
        .trim()
        .split_whitespace()
        .map(|s| {
            let mut iter = s.split(',');
            (
                iter.next().unwrap().parse().expect("Invalid input"),
                iter.next().unwrap().parse().expect("Invalid input"),
            )
        })
        .collect();
    let res = DSA::range_sum_queries(m, n, arr.clone(), queries.clone());
    println!("{:?}", arr);
    for i in 0..n {
        println!("{:?} - {}", queries[i], res[i]);
    }
}
