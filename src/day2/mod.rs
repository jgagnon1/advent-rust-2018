use std::io;
use std::collections::HashMap;
use itertools::Itertools;
use util::read_file;

pub fn solve() -> Result<(), io::Error> {
	let input = read_file("./src/day2/input.txt")?;

	let data: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect_vec()).collect();

	let frequency_maps = data.iter().map(|code| {
		code.iter()
			// Build map c -> frequency
			.fold(HashMap::new(), |mut acc, c| {
				*acc.entry(*c).or_insert(0) += 1;
				acc
			})
	}).collect_vec();

	// Compile values only once with (two, threes) occurrence tuple
	let appearance_tuple = frequency_maps.iter().map(|m| {
		m.values()
			.fold((0, 0), |(two, three), b| {
				if *b == 2 {
					(1, three)
				} else if *b == 3 {
					(two, 1)
				} else {
					(two, three)
				}
			})
	});

	// Compile all twos and threes by adding them
	let (twos, threes) = appearance_tuple.fold((0, 0), |(aa, bb), (a, b)| (aa + a, bb + b));
	let checksum1 = twos * threes;

	println!("[Part 1] Checksum is : {}", checksum1.to_string());

	let (a, b) = data.iter().find_map(|curr| {
		data.iter().find(|other| {
			let diff = curr.iter()
				.zip(other.iter())
				.fold(0, |acc, (a, b)| {
					if a == b { acc } else { acc + 1 }
				});
			diff == 1
		}).map(|f| (f, curr))
	}).expect("Code not found");

	// TODO : Extract the char diff between 2 strings
	let s1: String = a.iter().collect();
	let s2: String = b.iter().collect();
	println!("[Part 2] Codes matching (1diff) : {} & {}", s1, s2);

	Ok(())
}
