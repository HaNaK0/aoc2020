use vector2d::Vector2D;

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

fn get_neighbors(index: usize, map: &SeatMap) -> Vec<usize> {
	let dirs = [
		Vector2D::new(0, -1),
		Vector2D::new(0, 1), 
		Vector2D::new(1, -1),
		Vector2D::new(-1, 1), 
		Vector2D::new(-1,-1),
		Vector2D::new(1, 1),
		Vector2D::new(1, 0),
		Vector2D::new(-1, 0),
	];

	let xy_pos = Vector2D::new((index % MAP_WIDTH) as i32, (index / MAP_WIDTH) as i32);

	let indecies = dirs.iter().filter_map(|v| cast_ray(xy_pos, *v, map));

	indecies.collect()
}

fn cast_ray(start: Vector2D<i32>, dir: Vector2D<i32>, map: &SeatMap) -> Option<usize> {

	for i in 0..MAP_WIDTH{
		let current_pos: Vector2D<i32> = start + dir * i as i32;

		if current_pos.x >= MAP_WIDTH as i32 || current_pos.x < 0 || current_pos.y >= MAP_WIDTH as i32 || current_pos.y < 0 {
			return None;
		}

		let index = current_pos.x + current_pos.y * MAP_WIDTH as i32;

		if map[index as usize] != SeatStatus::Floor {
			return Some(index as usize);
		}
	}

	panic!()
}

// Returns the next iteration of the game if a change
fn get_next_iter(prev: &SeatMap, neighbors: &Vec<Vec<usize>>) -> Option<SeatMap> {
	let mut new_map = 

	todo!()
}

pub(crate) fn run() -> () {
	let mut seats = read_seats();
	println!("Seats loaded");

	let seat_neigbors: Vec<Vec<usize>> = (0..seats.len()).map(|i| get_neighbors(i, &seats)).collect();
	println!("Neigbors Found");

	while let Some(next) = get_next_iter(&seats, &seat_neigbors) {
		seats = next;
	}
	
	let result = seats.iter().filter(|&&s| s == SeatStatus::Taken).count();

	println!("Number of occupied seats {}", result)
}