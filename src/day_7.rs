use std::collections::HashMap;

use crate::read_lines::read_day;

type BagMap = HashMap<String, Vec<(usize, String)>>;

fn read_bags() -> BagMap {
	let lines = read_day(7).map(|l| l.unwrap());
	let mut bag_map = HashMap::new();

	for line in lines {
		read_bag(line, &mut bag_map);
	}

	bag_map
}

fn read_bag(line: String, bag_map: &mut BagMap) {
	let words: Vec<&str> = line.split(" ").collect();

	let bag_name = words[0..2].join(" ");

	let mut bag_vec = Vec::new();

	for i in (4..words.len()).step_by(4) {
		let sub_bag = words[i+1..i+3].join(" ");

		if sub_bag != "other bags." {
			bag_vec.push((words[i].parse().unwrap() ,sub_bag));
		}
	}

	bag_map.insert(bag_name, bag_vec);
}

fn count_parents(bag: &str, bag_map: &BagMap ) -> usize {
	let mut count = 0;
	for target_bag in bag_map {
		if is_bag_in_target(&target_bag.0, bag, &bag_map) {
			count += 1;
		}
	}

	count
}

//search for bag in target
fn is_bag_in_target(target: &String, bag: &str, bag_map: &BagMap, ) -> bool {
	//println!("{}", target);
	if let Some(_) = bag_map[target].iter().find(|x| x.1 == bag) {
		return true;
	}

	for bag_tup in &bag_map[target] {
	
		if is_bag_in_target(&bag_tup.1, bag, &bag_map) {
			return true;
		}
	}
	false
}

fn count_children(this_bag: &str, bag_map: &BagMap) -> usize {
	let this_bag = &bag_map[this_bag];
	this_bag.iter().map(|b| b.0 + b.0 * count_children(&b.1, bag_map)).sum::<usize>()
}

pub fn run() {
	let bag_map = read_bags();

	let count = count_parents("shiny gold", &bag_map);
	println!("Number of bags that can hold yours:{}", count);

	let count = count_children("shiny gold", &bag_map);
	println!("Number of bags in your bag:{}", count);
}