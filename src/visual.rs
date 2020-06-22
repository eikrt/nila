pub mod visual{
extern crate term;
use std::io::prelude::*;
use std::fs;


	pub fn visualize() {
		let paths = fs::read_dir("/").unwrap();
		let mut t = term::stdout().unwrap();
		t.fg(term::color::GREEN).unwrap();


		for path in paths {
	
			writeln!(t,"name: {}", path.unwrap().path().display()).unwrap();
		}

		t.reset().unwrap();
	}
  }
