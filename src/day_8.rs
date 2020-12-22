use core::panic;

use crate::read_lines::read_day;

#[derive(Clone)]
enum Instruction {
	Nop(i32),
	Acc(i32),
	Jmp(i32),
}

fn read_instructions(day_number: usize) -> Vec<Instruction> {
	read_day(day_number).map(|l|{
		let line = l.unwrap();
		let mut line = line.split(" ");

		let op = line.next().unwrap();
		let arg =line.next().unwrap().parse::<i32>().unwrap();

		match op {
			"nop" => Instruction::Nop(arg),
			"acc" => Instruction::Acc(arg),
			"jmp" => Instruction::Jmp(arg),
			_ => panic!("Unrecognised instruction")
		}
	}).collect()
}

//runs until it loop and returns the instructions that was visited
fn run_instructions(instructions: &Vec<Instruction>) -> Result<i32, Vec<bool>> {
	let mut visited = vec![false; instructions.len()];

	let mut inst : i32 = 0;
	let mut acc = 0;

	while !visited[inst as usize] {
		visited[inst as usize] = true;
		match instructions[inst as usize] {
		    Instruction::Nop(_) => {
				inst += 1;
			}
		    Instruction::Acc(arg) => {
				acc += arg;
				inst += 1;
			},
		    Instruction::Jmp(arg) => {
				inst += arg
			},
		}
		assert!(inst >= 0);
		if inst as usize == instructions.len() {
			return Ok(acc);
		}
	}
	Err(visited)
}

pub fn run() {
	let instructions = read_instructions(8);
	let visited = run_instructions(&instructions).unwrap_err();

	let potential_faults = visited.iter().enumerate().filter_map(|e| {
		if *e.1 {
			Some(e.0)
		} else {
			None
		}
	}).filter(|i| {
		match instructions[*i] {
			Instruction::Jmp(_) => true,
		    Instruction::Nop(_) => true,
		    Instruction::Acc(_) => false,
		}
	});

	for fault in potential_faults {
		let mut corrected_instructions = instructions.to_vec();

		corrected_instructions[fault] = match instructions[fault] {
		    Instruction::Nop(arg) => Instruction::Jmp(arg),
		    Instruction::Acc(_) => panic!(),
		    Instruction::Jmp(arg) => Instruction::Nop(arg),
		};

		match run_instructions(&corrected_instructions) {
		    Ok(acc) => {
				println!("acc {}", acc);
				break;
			}
		    Err(_) => {}
		};
	}
	println!("done")
}
