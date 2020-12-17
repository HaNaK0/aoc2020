use std::{collections::HashMap, fs::File, io};

use crate::read_lines::read_day;

const LETTERS : &str = "abcdefghijklmnopqrstuvxyz";
const DIDGITS : &str = "1234567890";
const COLORS : &str = "amb blu brn gry grn hzl oth";

struct Passports {
	lines: io::Lines<io::BufReader<File>>,
}

impl Passports {
	fn new(lines: io::Lines<io::BufReader<File>>) -> Self {
		Passports{lines}
	}
}

impl Iterator for Passports {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
		let mut passport = self.lines.next()?.unwrap();
		
		while let Some(line) = self.lines.next() {
			let line = line.unwrap();
			if line.len() > 0 {
				passport = format!("{} {}", passport, line)
			} else {
				break;
			}
		}
		Some(passport)
    }
}

fn check_passport(passport: &str) -> Option<()>{
	let passport_map : HashMap<_, _> = passport.split(" ").map(|f| {
		let mut iter = f.split(":");
		(iter.next().unwrap(), iter.next().unwrap())
	}).collect();

	if !passport_map.contains_key("byr") || !check_number(passport_map["byr"], 1920, 2002) {
		return None;
	}
	if !passport_map.contains_key("iyr") || !check_number(passport_map["iyr"], 2010, 2020) {
		return None;
	}
	if !passport_map.contains_key("eyr") || !check_number(passport_map["eyr"], 2020, 2030) {
		return None;
	}

	if !passport_map.contains_key("hgt") || !check_hgt(passport_map["hgt"]) {
		return None;
	}

	if !passport_map.contains_key("hcl") || !check_hcl(passport_map["hcl"]) {
		return None;
	}

	if !passport_map.contains_key("ecl") || !check_ecl(passport_map["ecl"]) {
		return None;
	}

	if !passport_map.contains_key("pid") || !check_pid(passport_map["pid"]) {
		return None;
	}


	Some(())
}

fn check_number(number: &str, min: i32, max: i32) -> bool {
	match number.parse::<i32>() {
		Ok(number) => number >= min && number <= max,
		Err(_) => false,
	}
}

fn check_hgt(hgt: &str) -> bool{
	if let Some(i) = hgt.find("in") {
		return check_number(&hgt[..i], 59, 76);
	}
	if let Some(i) = hgt.find("cm") {
		return check_number(&hgt[..i], 150, 193);
	}

	false
}

fn check_hcl(hcl: &str) -> bool {
	let mut chars = hcl.chars();
	if chars.next() != Some('#') {
		return false;
	}

	let mut count = 0;
	for letter in chars {
		count += 1;
		if !(LETTERS.contains(letter) || DIDGITS.contains(letter)) {
			return false;
		}
	}

	return count == 6;
}

fn check_ecl(ecl: &str) -> bool {
	COLORS.contains(ecl)	
}

fn check_pid(pid: &str) -> bool {
	let mut count = 0;

	for didgit in pid.chars() {
		count += 1;
		if !DIDGITS.contains(didgit) {
			return false;
		}
	}

	return count == 9;
}

pub fn run()
{
	let pass_iter = Passports::new(read_day(4));

	let valid_passport_count = pass_iter.filter_map(|p| check_passport(&p)).count();
	println!("valid passports:{}", valid_passport_count)
}