use crate::read_lines::{read_day};

const MAP_WIDTH : usize = 98;

#[derive(PartialEq, Copy, Clone)]
enum SeatStatus {
	Free,
	Taken,
	Floor,
}

type SeatMap = Vec<SeatStatus>;

fn read_seats() -> SeatMap{
	let lines = read_day(11).map(|l| l.unwrap());

	let mut map = SeatMap::with_capacity(MAP_WIDTH * 98);

	for line in lines {
		for char in line.chars() {
			match char {
				'.' => map.push(SeatStatus::Floor),
				'L' => map.push(SeatStatus::Free),
				'#' => map.push(SeatStatus::Taken),
				_ => panic!("Wrong!"),
			}
		}
	}
	
	map
}

fn get_neighbors(index: usize, map: &SeatMap) -> SeatMap {
	let relative_neigbors = [(-1,-1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
	let xy_pos = ((index % MAP_WIDTH) as i32, (index / MAP_WIDTH) as i32);
	let indecies = relative_neigbors.iter().
		map(|p| (xy_pos.0 + p.0, xy_pos.1 + p.1)).
		filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < MAP_WIDTH as i32 && p.1 < MAP_WIDTH as i32).
		map(|p| p.0 as usize + p.1 as usize * MAP_WIDTH);

	indecies.map(|i| map[i]).collect()
}

// Returns the next iteration of the game if a change
fn get_next_iter(prev: &SeatMap) -> Option<SeatMap> {
	let mut change = false;
	let iter = prev.iter().
		enumerate().
		map(|s| {
			let neighbors = get_neighbors(s.0, prev);
			match s.1 {
			    SeatStatus::Free => {
					let taken_neigbors = neighbors.iter().filter(|&&s| s == SeatStatus::Taken);
					if taken_neigbors.count() == 0 {
						change = true; 
						SeatStatus::Taken
					} else {
						SeatStatus::Free
					}
				}
			    SeatStatus::Taken => {
					let taken_neigbors = neighbors.iter().filter(|&&s| s == SeatStatus::Taken);
					if taken_neigbors.count() >= 4 {
						change = true;
						SeatStatus::Free
					} else {
						SeatStatus::Taken
					}
				}
			    SeatStatus::Floor => SeatStatus::Floor 
			}
		});
		
		let new_map = iter.collect();

		if change {
			Some(new_map)
		} else {
			None
		}
}

pub(crate) fn run() -> () {
	let mut seats = read_seats();

	while let Some(next) = get_next_iter(&seats) {
		seats = next;
	}
	
	let result = seats.iter().filter(|&&s| s == SeatStatus::Taken).count();

	println!("Number of occupied seats {}", result)
}