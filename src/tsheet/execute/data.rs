use super::clock::Clock;

pub struct Data {
	pub date: String,
	pub work_order: String,
	pub time_in: Clock,
	pub time_out: Clock,
}

impl Data {
	const L_COLS_EXPECTED: usize = 13;
	const S_COLS_EXPECTED: usize = 12;
	const COL_DATE      : usize =  1;
	const COL_WORK_ORDER: usize =  4;
	const COL_SEQUENCE  : usize =  5;
	const COL_TIME_IN   : usize =  7;
	const COL_TIME_OUT  : usize =  8;

	pub fn new(line: &str) -> Option<Data> {
		let mut list = Self::collect_fields(line);
		
		match list.len() {
			Data::L_COLS_EXPECTED => {
				Self::combine_work_order(&mut list);
			},
			Data::S_COLS_EXPECTED => {
				()
			},
			_ => {
				return None;
			},
		}

		if let Some(time_in) = Clock::new_from_string(&list[Data::COL_TIME_IN]) {
			if let Some(time_out) = Clock::new_from_string(&list[Data::COL_TIME_OUT]) {
				return Some(
					Data {
						date      : list[Data::COL_DATE].to_string(),
						work_order: list[Data::COL_WORK_ORDER].to_string(),
						time_in   : time_in,
						time_out  : time_out,
					}
				);
			}
		}
		
		None
	}
	
	pub fn date(&self) -> &str {
		&self.date
	}
	
	pub fn work_order(&self) -> &str {
		&self.work_order
	}
	
	pub fn time_in(&self) -> &Clock {
		&self.time_in
	}
	
	pub fn time_out(&self) -> &Clock {
		&self.time_out
	}
/*
	pub fn get_as_string(&self) -> String {
		format![
			"date: '{}'\torder: '{}'\tin: '{}'\tout: '{}'",
			self.date,
			self.work_order,
			self.time_in.as_string(),
			self.time_out.as_string(),
		]
	}
*/
	fn collect_fields(s: &str) -> Vec<String> {
		let mut v: Vec<String> = Vec::new();
		
		for word in s.split_whitespace() {
			v.push(word.to_string());
		}
		
		v
	}

	fn combine_work_order(v: &mut Vec<String>) {
		let s = v[Data::COL_SEQUENCE].clone();
		
		v[Data::COL_WORK_ORDER].push_str(&s);
		v.remove(Data::COL_SEQUENCE);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn data_long_test() {
		let test_input = "  40 06/29/23 LASTNAME, F     280990-  1   10    1    10:26:13  10:50:51    0.41  N/A     1";
		let test_data = Data::new(test_input).unwrap();
		let test_clock_1 = Clock::new(10, 26, 13);
		let test_clock_2 = Clock::new(10, 50, 51);
		
		assert!(
			test_data.date() == "06/29/23" &&
			test_data.work_order() == "280990-1" &&
			test_data.time_in.h() == test_clock_1.h() &&
			test_data.time_in.m() == test_clock_1.m() &&
			test_data.time_in.s() == test_clock_1.s() &&
			test_data.time_out.h() == test_clock_2.h() &&
			test_data.time_out.m() == test_clock_2.m() &&
			test_data.time_out.s() == test_clock_2.s()
		);
	}
	
	#[test]
	fn data_short_test() {
		let test_input = "  50 07/04/23 LASTNAME, F     55555-550    10    1    10:50:51  15:56:43    4.85  N/A     1";
		let test_data = Data::new(test_input).unwrap();
		let test_clock_1 = Clock::new(10, 50, 51);
		let test_clock_2 = Clock::new(15, 56, 43);
		
		assert!(
			test_data.date() == "07/04/23" &&
			test_data.work_order() == "55555-550" &&
			test_data.time_in.h() == test_clock_1.h() &&
			test_data.time_in.m() == test_clock_1.m() &&
			test_data.time_in.s() == test_clock_1.s() &&
			test_data.time_out.h() == test_clock_2.h() &&
			test_data.time_out.m() == test_clock_2.m() &&
			test_data.time_out.s() == test_clock_2.s()
		);
	}
	
	#[test]
	fn data_invalid_test() {
		let test_input = "  aBcDe FgHiJ";
		
		if let Some(_test_data) = Data::new(test_input) {
			assert!(false);
		} else {
			assert!(true);
		}
	}
}