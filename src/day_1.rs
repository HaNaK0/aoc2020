use std::env::current_dir;

use crate::read_lines::read_lines;

fn puzzle_1(numbers: &Vec<i32>) -> Option<i32> {
	for n_1 in numbers {
		for n_2 in numbers{
			if n_1 + n_2 == 2020 {
				return Some(n_1 * n_2);
			}
		}
	}

	None
}

fn puzzle_2(numbers: &Vec<i32>) -> Option<i32> {
	for n_1 in numbers {
		for n_2 in numbers{
			for n_3 in numbers{
				if n_1 + n_2 + n_3 == 2020 {
					return Some(n_1 * n_2 * n_3);
				}
			}
		}
	}

	None
}

pub fn run() {
	let mut numbers = Vec::new();
	let root_dir = current_dir().unwrap();
	let puzzle_path = root_dir.join("Input").join("day_1").join("puzzle_1.txt");
	
	match read_lines(puzzle_path) {
		Ok(lines) => {
			for line in lines {
				if let Ok(line) = line {
					numbers.push(line.parse::<i32>().unwrap())
				}
			}
		},
		Err(err) =>{
			println!("Something went wrong: {}", err);
			panic!();
		}
	}

	println!("first result {}", puzzle_1(&numbers).unwrap());
	println!("second result {}", puzzle_2(&numbers).unwrap());
}