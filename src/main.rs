mod visual;
use std::io;
use std::process;
fn main() {


	let mut running = true;
	let mut input;
	let mut path = String::from("/*");
	while running {
		println!("{}", path);
		visual::visualize(&mut path);
		input = String::new();
		io::stdin().read_line(&mut input)
			.ok()
			.expect("Failed to read  line!");	
		
		if input.chars().next().unwrap() == ':'{
			
			


				if input == ":q"{
					process::exit(1)
				}
			
				else if &input[0..3] == ":cd"{
		
					if input.len() > 3 {
						path =input[4..input.len()].to_string();

					}
				}
				else {
					//log command not found
				}


		}	

		
		
	}
}
