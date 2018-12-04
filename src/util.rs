use std::io::{self, Read};
use std::fs::File;

pub fn read_file(path: &str) -> Result<String, io::Error> {
	let mut file = File::open(path)?;
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	Ok(s)
}
