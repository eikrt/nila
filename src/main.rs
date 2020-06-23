mod visual;
use std::io;
use std::process;
fn main() {


	let mut running = true;
	let mut input;
	let mut path = String::from("/");
	while running {

		visual::visualize(&mut path);
		input = String::new();
		io::stdin().read_line(&mut input)
			.ok()
			.expect("Failed to read  line!");	
		
		if input.chars().next().unwrap() == ':'{
			
			

				if input.trim() == ":q"{
					process::exit(1)
				}
			



		}	

		
		
	}
}
