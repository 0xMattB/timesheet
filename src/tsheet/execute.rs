mod data;
mod clock;
mod report;

use data::Data;

pub fn execute(filedata: &str) -> String {
	let mut data: Vec<Data> = Vec::new();
	
	for line in filedata.lines() {
		if let Some(d) = Data::new(line) {
			data.push(d);
		}
	}
	
	report::run(&data)
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn execute_test() {
		let test_input = "\
40 06/29/23 LASTNAME, F     280990-  1   10    1    10:26:13  10:50:51    0.41  N/A     1
50 06/29/23 LASTNAME, F     55555-550    10    1    10:50:51  15:56:43    4.85  N/A     1";
		
		let test_output = "\
280990-1\t(description)\t0.50
55555-550\t(description)\t5.00
";
		
		assert_eq!(
			execute(test_input),
			test_output.to_string()
		);
	}
}