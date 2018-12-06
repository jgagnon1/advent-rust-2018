use std::io::{self, BufRead};
use std::collections::HashMap;
use chrono::prelude::*;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Record {
	dt: DateTime<Utc>,
	entry: String,
}

// Disclaimer : I really don't like how this one turned out - should refactor / or just use it as exercise to improve going forward.
pub fn solve() -> Result<(), io::Error> {
	let stdin = io::stdin();
	let data: Vec<String> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect::<Vec<_>>();

	let re = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (.*)$").unwrap();

	let records = data.iter().filter_map(|entry| {
		re.captures_iter(entry).map(|cap| {
			let dt = Utc.datetime_from_str(&cap[1], "%Y-%m-%d %H:%M").unwrap();
			let entry = cap[2].to_string();
			Record { dt, entry }
		}).next()
	}).collect::<Vec<_>>();

	let sorted = records.iter()
		.sorted_by(|a, b| a.dt.cmp(&b.dt));

	// TODO : Keep all hashmaps into 1 struct - Not too happy about all I have to carry through the fold, specially since mutable (defeats the point?)
	let (_, _, overall_mins, overall_freq_mins, overall_freq_mins_count): (Option<u32>, Option<DateTime<Utc>>, HashMap<u32, u32>, HashMap<u32, HashMap<u32, u32>>, HashMap<u32, HashMap<u32, u32>>) =
		sorted.iter()
			.fold((None, None, HashMap::new(), HashMap::new(), HashMap::new()),
				  |(current, asleep, mut total_mins, mut freq_min, mut freq_min_count), Record { dt, entry }| {
					  if entry.ends_with("begins shift") {
						  let guard = str::parse(&entry.split(' ').nth(1).unwrap()[1..]).unwrap();
						  (Some(guard), None, total_mins, freq_min, freq_min_count)
					  } else {
						  match entry.as_str() {
							  "falls asleep" => {
								  (current, Some(*dt), total_mins, freq_min, freq_min_count)
							  }
							  "wakes up" => {
								  let guard_id = current.expect("No guard for wake up context.");
								  let start = asleep.expect("Guard did not fall asleep.");

								  let minutes = dt.signed_duration_since(start).num_minutes() as u32;
								  *total_mins.entry(guard_id).or_insert(0) += minutes;

								  for i in 0..minutes {
									  *freq_min
										  .entry(guard_id)
										  .or_default()
										  .entry(start.minute() + i)
										  .or_insert(0) += start.minute() + i;

									  *freq_min_count
										  .entry(start.minute() + i)
										  .or_default()
										  .entry(guard_id)
										  .or_insert(0) += 1;
								  }

								  (current, None, total_mins, freq_min, freq_min_count)
							  }
							  other => {
								  panic!("Invalid argument: {}", other);
							  }
						  }
					  }
				  });

	let (most_asleep_id, most_asleep_mins) = overall_mins.iter()
		.max_by(|(_, a), (_, b)| a.cmp(&b))
		.expect("No guard was the most asleep");

	let (most_freq_min, _) = overall_freq_mins
		.get(most_asleep_id)
		.expect("Guard not found in frequency map")
		.iter()
		.max_by(|(_, a), (_, b)| a.cmp(&b))
		.expect("No maximum minutes for guard.");

	println!("[Part 1]: Result {} - Most asleep guard ID : {} for {} minutes most often at minute : {}",
			 most_asleep_id * most_freq_min,
			 most_asleep_id, most_asleep_mins, most_freq_min);

	// min -> guard -> freq
	let (_, max_pair) = overall_freq_mins_count.iter().fold((None, None), |(max, max_pair), (cur_min, hash)| {
		let max_opt = hash.iter().max_by(|(_, a), (_, b)| a.cmp(&b));

		match max_opt {
			Some((guard_max, local_max)) => {
				if local_max > max.unwrap_or(&0) { // New max !
					(Some(local_max), Some((guard_max, cur_min)))
				} else {
					(max, max_pair)
				}
			}
			None => (max, max_pair)
		}
	});

	let (most_count_id, most_min) = max_pair.expect("No minute max found");

	println!("[Part 2]: Result {}", most_count_id * most_min);

	Ok(())
}
