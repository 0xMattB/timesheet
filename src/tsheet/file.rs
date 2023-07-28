use std::{
	io::Write,
	fs::{self, File},
	error::Error
};

pub fn read(filename: &str) -> Result<String, Box<dyn Error>> {
	let contents = fs::read_to_string(filename)?;
	
	Ok(contents)
}

pub fn write(filedata: &str) -> Result<(), Box<dyn Error>> {
	let mut file = File::create("output.txt")?;
	file.write_all(filedata.as_bytes())?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn file_read_test() {
		assert_eq!(read("test.txt").unwrap(), "test file.".to_string());
	}
}