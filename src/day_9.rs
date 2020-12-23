use crate::read_lines::read_day;

fn check_xmas(numbers : &Vec<u64>, len: usize) -> usize {
	println!("{} {}", len, numbers.len());
	for i in len..numbers.len() {
		if !check_xmas_number(&numbers[i - len..i], numbers[i]) {
			return i;
		}
	}
	panic!("did not find any numbers")
}

fn check_xmas_number(slice: &[u64], number: u64) -> bool{
	for n_1 in slice.iter().enumerate() {
		for n_2 in  &slice[n_1.0+1..]{
			if n_1.1 + n_2 == number {
				return true;
			}
		} 
	}
	false
}

fn find_xmas_brake(numbers : &Vec<u64>, fault_index: usize) -> Vec<u64> {
	let target = numbers[fault_index];

	for i in 0..fault_index {
		for j in (i+1)..fault_index{
			let sum = numbers[i..j+1].iter().sum::<u64>();
			if sum == target {
				return numbers[i..j+1].to_vec();
			}
		}
	}
	panic!("Failed to find a sum")
}

pub fn run() {
	let numbers = read_day(9).map(|l| {
		let string = l.unwrap();
		string.parse::<u64>().unwrap()
	});

	let numbers = numbers.collect();

	let index = check_xmas(&numbers, 25);
	let xmas_brake = find_xmas_brake(&numbers, index);
	println!("Number found {}", xmas_brake.iter().max().unwrap() + xmas_brake.iter().min().unwrap())
}