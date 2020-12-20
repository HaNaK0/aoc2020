use std::collections::{HashMap, HashSet, VecDeque};

use crate::read_lines::read_day;

fn read_bags() -> HashMap<String, Vec<String>> {
	let lines = read_day(7).map(|l| l.unwrap());
	let mut bag_map = HashMap::new();

	for line in lines {
		read_bag(line, &mut bag_map);
	}

	bag_map
}

fn read_bag(line: String, bag_map: &mut HashMap<String, Vec<String>>) {
	let mut words = line.split(" ");
	let mut current_bag = Vec::new();

	while let Some(word) = words.next() {
		if word == "bags" {
			break;
		}

		current_bag.push(word);
	}

	let current_bag = current_bag.join(" ");
	if let None = bag_map.get(&current_bag) {
		bag_map.insert(current_bag.clone(), Vec::new());
	}

	let mut contained_bag = Vec::new();
	while let Some(word) = words.next() {
		if let Ok(_) =  word.parse::<usize>() {
			continue;
		}

		if word == "bags," || word == "bags." {
			let bag = contained_bag.join(" ");
			if bag != "contain no other" {
				if bag_map.contains_key(&bag) {
					println!("psuh");
					let bag = bag_map.get_mut(&bag).unwrap();
					bag.push(current_bag.clone())
				} else {
					println!("insert");
					bag_map.insert(bag, vec![current_bag.clone()]);
				}
			}
			
			contained_bag.clear();
			continue;
		}

		contained_bag.push(word);
	}
}

fn count_parents(bag: &str, bag_map: HashMap<String, Vec<String>> ) -> usize {
	let mut result : HashSet<&String> = bag_map[bag].iter().collect();
	let mut queue : VecDeque<&String> = result.iter().cloned().collect();

	while let Some(bag) = queue.pop_back() {
		let bag = &bag_map[bag];
		let other : HashSet<&String> = bag.iter().collect();

		let diff : Vec<&String> = other.difference(&result).cloned().collect();
		for bag_name in diff {
			queue.push_front(bag_name);
			result.insert(bag_name);
		}
	}

	result.len()
}

pub fn run() {
	let bag_map = read_bags();
	
	let count = count_parents("shiny gold", bag_map);
	println!("Number of bags that can hold yours:{}", count)
}