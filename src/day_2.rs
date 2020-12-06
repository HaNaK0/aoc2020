use std::{num::ParseIntError, env::current_dir};

use crate::read_lines::read_lines;

pub fn run() {
	let root_dir = current_dir().unwrap();
	let puzzle_path = root_dir.join("Input").join("day_2.txt");

	let lines = read_lines(puzzle_path).unwrap();
	let mut count: usize = 0;
	for line in lines {
		if let Ok(line) = line {
			if check_password(line.as_str()) {
				count += 1;
			}
		}
	}

	println!("number of correct passwords: {}", count)
}

fn check_password(password: &str) -> bool {
	let mut iter = password.split(":");
	let rule = iter.next().unwrap();
	let password = iter.next().unwrap();

	let rule = Rule::from_string(rule).unwrap();

	let first_char = password.chars().nth(rule.lower);
	let first_match = match first_char {
		Some(first_char) => first_char == rule.letter,
		None => false,
	};

	let second_char = password.chars().nth(rule.upper);
	let second_match = match second_char {
		Some(second_char) => second_char == rule.letter,
		None => false,
	};

	return (first_match || second_match) && first_match != second_match;
}

struct Rule {
	upper: usize,
	lower: usize,
	letter: char,
}

impl Rule {
	fn from_string(rule_string: &str) -> Result<Self, ParseIntError> {
		let mut split = rule_string.split("-");
		let lower = split.next().unwrap().parse()?;
		let mut split = split.next().unwrap().split(" ");
		let upper = split.next().unwrap().parse()?;

		Ok(
			Rule {
				upper, 
				lower,
				letter: split.next().unwrap().chars().next().unwrap(),
			}
		)
	}
}