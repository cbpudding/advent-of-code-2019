use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_fuel(mass: usize) -> usize {
	if mass > 6 {
		let fuel = mass / 3 - 2;
		fuel + calculate_fuel(fuel)
	} else {
		0
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 2 {
		let input = File::open(&args[1]).unwrap();
		let reader = BufReader::new(input);
		let mut total = 0;
		for mass in reader.lines() {
			total += calculate_fuel(mass.unwrap().parse::<usize>().unwrap());
		}
		println!("Total Fuel Required: {}", total);
	} else {
		println!("Usage: {} <Input File>", args[0]);
	}
}

#[cfg(test)]
mod tests {
	use super::calculate_fuel;

	#[test]
	fn known_values() {
		assert_eq!(calculate_fuel(12), 2);
		assert_eq!(calculate_fuel(14), 2);
		assert_eq!(calculate_fuel(1969), 966);
		assert_eq!(calculate_fuel(100756), 50346);
	}
}
