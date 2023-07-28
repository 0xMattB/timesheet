mod tsheet;

use tsheet::arg;
use tsheet::file;
use tsheet::execute;
use std::{env, process};

const OUTPUT_FILENAME: &str = "output.txt";

pub fn run() {
	let filename = arg::parse(env::args().collect()).unwrap_or_else(|_| {
		eprintln!("usage: timesheet.exe (filename).txt");
		process::exit(1);
	});
	
	let filedata = file::read(&filename).unwrap_or_else(|err| {
		eprintln!("error opening {filename}: {err}");
		process::exit(1);
	});
	
	match file::write(&execute::execute(&filedata)) {
		Ok(()) => { println!("\nData written to '{OUTPUT_FILENAME}'\n"); },
		Err(e) => { println!("\nError writing data to '{OUTPUT_FILENAME}': {}", e); },
	}
	
	println!("{}", execute::execute(&filedata));	
}
