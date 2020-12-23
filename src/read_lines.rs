use std::{fs::File, env::current_dir};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_day(day_number: usize) -> io::Lines<io::BufReader<File>> {
	let root_dir = current_dir().unwrap();
	let puzzle_path = root_dir.join("Input").join(format!("day_{}.txt", day_number));

	match read_lines(puzzle_path) {
		Ok(lines) => lines,
		Err(error) => {
			eprintln!("Failed to load input: {}", error);
			panic!();
		}
	}
}

pub fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}