use std::collections::HashSet;

use crate::read_lines::read_day;

const LINE_WIDTH: usize = 31;

type Map = Vec<HashSet<usize>>;


pub fn run() {
	let map = read_map();

	let mut tree_count = check_slope(&map, (1,1));
	tree_count *= check_slope(&map, (3,1));
	tree_count *= check_slope(&map, (5,1));
	tree_count *= check_slope(&map, (7,1));
	tree_count *= check_slope(&map, (1,2));

	println!("tree count:{}", tree_count)
}

fn check_slope(map: &Map, slope: (usize, usize)) -> usize {
	let mut tree_count = 0;
	for i in (0..map.len()).step_by(slope.1) {
		if map[i].contains(&((i / slope.1 * slope.0) % LINE_WIDTH)) {
			tree_count += 1;
		}
	}
	tree_count
}

fn read_map()-> Map{
	let lines = read_day(3);

	let mut map = Vec::new();

	for line in lines {
		let line = line.unwrap();
		let matches = line.match_indices("#");
		let mut tree_set = HashSet::new();

		for match_index in matches {
			tree_set.insert(match_index.0);
		}

		map.push(tree_set)
	};

	map
}