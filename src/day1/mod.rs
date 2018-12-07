use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, BufRead};

pub fn solve() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let data: Vec<i32> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let sum1: i32 = data.iter().sum();
    println!("[Part 1] Result frequency is : {}", sum1.to_string());

    let (sum2, _) = data
        .iter()
        .cycle()
        .fold_while((0, HashSet::new()), |(acc, mut state), b| {
            state.insert(acc);
            let next = acc + b;
            if state.contains(&next) {
                Done((next, state))
            } else {
                Continue((next, state))
            }
        })
        .into_inner();
    println!(
        "[Part 2] First frequency reached twice is: {}",
        sum2.to_string()
    );

    Ok(())
}
