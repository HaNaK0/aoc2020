use std::{fs::File, collections::HashSet, io};

use crate::read_lines::read_day;


pub fn run() {
	let mut lines = read_day(6);

	let mut sum = 0;
	while let Some(answer_count) = check_group(&mut lines) {
		println!("{}", answer_count);
		sum += answer_count;
	}

	println!("number of yeses:{}", sum)
}

fn check_group(lines: &mut io::Lines<io::BufReader<File>>) -> Option<usize> {
	let mut line = lines.next()?.unwrap();
	let mut result : HashSet<char> = line.chars().collect();

	loop{
		line = lines.next().unwrap_or(Ok("".to_string())).unwrap();
		if line == "" {
			break;
		}

		let next : HashSet<char> =  line.chars().collect();
		result = result.intersection(&next).map(|s| s.clone()).collect();
	}
	
	Some(result.len())
}