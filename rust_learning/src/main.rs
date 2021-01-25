use std::io;

fn for_test(n: i32) -> i32
{
	let i: i32 = 0;
	for i in 0..n {
		println!("Current value: {}", i);
	}
	i
}

/*
	match io::stdin().read_line(&mut input) {
		Ok(_) => {
			//let val: u32 = from_str_radix(&input, 10);
			let val: i32 = input.parse().unwrap();
			println!("Input was: {}", val);
			let lv = for_test(val);
			println!("Last val: {}", lv);
		}
		Err(error) => println!("error: {}", error),
	}

*/

fn main() {
	println!("Max Iternation:");
	let mut input = String::new();
	io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
	println!("Input string was: {}", input);
	let input: i32 = input.trim().parse()
		.ok()
		.expect("Please type a number!");
	println!("Input was: {}", input);
	let lv = for_test(input);
	println!("Last val: {}", lv);
}
