extern crate term;
use std::io::prelude::*;
use std::fs;

	pub fn visualize(pwd: &String) {
		let paths = fs::read_dir(pwd).unwrap();
		let mut t = term::stdout().unwrap();
		t.bg(term::color::WHITE).unwrap();
		t.fg(term::color::BLUE).unwrap();
		writeln!(t,"Currently in directory: {}", pwd);

		t.fg(term::color::GREEN).unwrap();
		for path in paths {
	
			writeln!(t,"name: {}", path.unwrap().path().display()).unwrap();
		}

		t.reset().unwrap();
	}
