pub fn parse(v: Vec<String>) -> Result<String, ()> {
	if v.len() != 2 {
		Err(())
	} else {
		Ok(v[1].clone())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn arg_parse_test() {
		let v = vec![
			String::from("zero"),
		];
		
		assert_eq!(parse(v), Err(()));
		
		let v = vec![
			String::from("zero"),
			String::from("one"),
		];
		
		assert_eq!(parse(v), Ok("one".to_string()));
	}
}