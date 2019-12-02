use std::env;
use std::fs::File;
use std::io::Read;

fn execute(memory: Vec<isize>) -> isize {
	let mut mem = memory.clone();
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
		println!("memory[0] = {}", execute(memory));
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
