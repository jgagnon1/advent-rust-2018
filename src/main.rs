extern crate chrono;
extern crate clap;
extern crate itertools;
extern crate regex;

use clap::{App, Arg};
use std::io;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

fn main() -> Result<(), io::Error> {
    let matches = App::new("Advent of Rust 2018")
        .author("Jerome Gagnon <jerome.gagnon.1@gmail.com>")
        .arg(
            Arg::with_name("day")
                .required(true)
                .help("Day of the advent calendar")
                .validator(|str| {
                    str.parse::<u32>()
                        .or_else(|_| Err("day must be an integer".to_owned()))
                        .and_then(|v| match v {
                            1...25 => Ok(()),
                            _ => Err("day must be between 1 and 25".to_owned()),
                        })
                }),
        )
        .get_matches();

    match matches.value_of("day").unwrap().parse::<u32>().unwrap() {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        _ => Ok(()),
    }
}
