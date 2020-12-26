use crate::read_lines::read_day;

type ResultType = u64;

pub fn run() {
	let lines = read_day(10).map(|l| l.unwrap());
	let adapters : Vec<i32> = lines.map(|l| l.parse::<i32>().unwrap()).collect();

	let steps = create_adaptor_chain(&adapters);
	println!("The step counts were: {:?}", steps);
	println!("The rsult should be: {}", count_adapter_combinations(&adapters));
}

fn create_adaptor_chain(adapters: &Vec<i32>) -> [usize; 3] {
	let mut steps = [0, 0, 1]; // where the position coresponds to the step size and the built in addapter is added

	let mut sorted_adapters = adapters.clone();
	sorted_adapters.sort();
	
	let mut current_joltage = 0;
	for adapter in sorted_adapters {

		let step = adapter - current_joltage;

		steps[(step - 1) as usize] += 1;
		current_joltage = adapter;
	}

	return steps;
}

fn count_adapter_combinations(adapters: &Vec<i32>) -> ResultType {
	let mut sorted_adapters = adapters.clone();
	sorted_adapters.sort();

	let mut path_counts = vec![0 as ResultType; sorted_adapters.len()];
	path_counts[sorted_adapters.len() - 1] = 1;

	for i in (0..sorted_adapters.len() - 1).rev() {
		let current_adapter = adapters[i];

		let count = sorted_adapters[i+1..].iter()
			.take_while(|&j| j - current_adapter <= 3).count();

		let path_len : ResultType = path_counts[i+1..i+count+1].iter().sum();
		path_counts[i] = path_len;
	}
	path_counts[0]
}