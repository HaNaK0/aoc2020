use crate::read_lines;



pub fn run() {
	let lines = read_lines::read_day(5);

	let mut ids : Vec<usize> = lines.map(|s| get_id(s.unwrap().as_str())).collect();
	ids.sort();
	let seat = find_empty_seat(ids).unwrap();

	println!("your seat:{}", seat);
}

fn get_id(seat: &str) -> usize {
	//First get the row
	let mut low = 0;
	let mut high = 127;

	for letter in seat[..7].chars() {
		if letter == 'F' {
			high = low + (high - low) / 2;
		} else if letter == 'B' {
			low = high - (high - low) / 2
		} else {
			panic!()
		}
	}
	assert_eq!(low, high);
	let row = high;

	//then get the seat number
	let mut low = 0;
	let mut high = 7;

	for letter in seat[7..].chars() {
		if letter == 'L' {
			high = low + (high - low) / 2;
		} else if letter == 'R' {
			low = high - (high - low) / 2
		} else {
			panic!()
		}
	}
	assert_eq!(low, high);
	let column = high;

	row * 8 + column
}

fn find_empty_seat(ids: Vec<usize>) -> Option<usize> {
	for i in 1..ids.len() {
		assert_ne!(ids[i], ids[i-1]);
		if ids[i] - ids[i - 1] > 1 {
			return Some(ids[i] - 1);
		}
	}

	None
}