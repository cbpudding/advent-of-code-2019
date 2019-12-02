use std::env;
use std::fs::File;
use std::io::Read;

fn execute(mut mem: Vec<isize>) -> isize {
	let mut ptr = 0;
	while mem[ptr] != 99 {
		let (a, b, c) = (mem[ptr + 1] as usize, mem[ptr + 2] as usize, mem[ptr + 3] as usize);
		match mem[ptr] {
			1 => {
				mem[c] = mem[a] + mem[b];
			},
			2 => {
				mem[c] = mem[a] * mem[b];
			},
			_ => {
				println!("Invalid opcode: {}!", mem[ptr]);
			}
		}
		ptr += 4;
	}
	mem[0]
}

fn solve(memory: Vec<isize>, desired: isize) -> (isize, isize) {
	let mut program = memory.clone();
	program[1] = 0;
	program[2] = 0;
	let offset = execute(program.clone());
	let raw = desired - offset;
	program[1] = 1;
	let multiplier = execute(program.clone()) - offset;
	(raw / multiplier, raw % multiplier)
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 2 {
		let mut program = File::open(&args[1]).unwrap();
		let mut memory: Vec<isize> = Vec::new();
		let mut data = String::new();
		program.read_to_string(&mut data).unwrap();
		for cell in data.split(",") {
			memory.push(String::from(cell).trim().parse::<isize>().unwrap());
		}
		let (x, y) = solve(memory, 19690720);
		println!("Solution: ({}, {})", x, y);
	} else {
		println!("Usage: {} <Input File>", args[0]);
	}
}

#[cfg(test)]
mod tests {
	use super::execute;

	#[test]
	fn known_values() {
		assert_eq!(execute(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]), 3500);
	}
}
