use std::io;
use std::collections::HashSet;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use util::read_file;

pub fn solve() -> Result<(), io::Error> {
	let input = read_file("./src/day1/input.txt")?;

	let data: Vec<i32> = input.lines().filter_map(|s| s.parse::<i32>().ok()).collect();

	let sum1: i32 = data.iter().sum();
	println!("[Part 1] Result frequency is : {}", sum1.to_string());

	let (sum2, _) = data.iter()
		.cycle()
		.fold_while((0, HashSet::new()), |(acc, mut state), b| {
			state.insert(acc);
			let next = acc + b;
			if state.contains(&next) { Done((next, state)) } else {
				Continue((next, state))
			}
		})
		.into_inner();
	println!("[Part 2] First frequency reached twice is: {}", sum2.to_string());

	Ok(())
}
