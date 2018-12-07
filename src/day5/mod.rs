use std::io::{self, BufRead};

// Disclaimer: Tried to leave Itertools aside for this one and take a more rusty approach ;)
pub fn solve() -> Result<(), io::Error> {
	let stdin = io::stdin();
	let data = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.next()
		.unwrap()
		.chars().collect::<Vec<_>>();

	let reacted_chain = react(data.iter());

	println!("[Part 1]: Result {}", reacted_chain.len());

	let mut min = std::usize::MAX;
	for i in 0u8..=26 {
		let v = reacted_chain.iter().filter(|&&c| c as u8 != b'a'+i && c as u8 != b'A'+i);
		min = usize::min(min, react(v).len())
	}

	println!("[Part 2]: Result {}", min);

	Ok(())
}

fn react<'a>(chain: impl Iterator<Item = &'a char>) -> Vec<char> {
	let mut v = Vec::new();
	for &c in chain {
		match v.last() {
			None => v.push(c),
			Some(&d) => if d.to_ascii_lowercase() == c.to_ascii_lowercase() && d != c {
				v.pop();
			} else {
				v.push(c);
			}
		}
	};
	v
}

