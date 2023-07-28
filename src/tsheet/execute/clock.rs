#[derive(Debug)]
pub struct Clock {
	h: usize,
	m: usize,
	s: usize,
}

#[allow(dead_code)]
impl Clock {
	pub fn new(h: usize, m: usize, s: usize) -> Clock {
		Clock {
			h: Self::h_set(h),
			m: Self::m_set(m),
			s: Self::s_set(s),
		}
	}
	
	pub fn new_from_string(t_str: &str) -> Option<Clock> {
		let c: Vec<_> = t_str.split(":").collect();
		
		if c.len() == 3 && Self::is_number(c[0]) && Self::is_number(c[1]) && Self::is_number(c[2]) {
			Some(
				Self::new(
					c[0].parse().unwrap(),
					c[1].parse().unwrap(),
					c[2].parse().unwrap(),
				)
			)
		} else {
			None
		}
	}
	
	pub fn h(&self) -> usize {
		self.h
	}
	
	pub fn m(&self) -> usize {
		self.m
	}
	
	pub fn s(&self) -> usize {
		self.s
	}

	pub fn as_string(&self) -> String {
		format!["{:02}:{:02}:{:02}", self.h, self.m, self.s]
	}
	
	pub fn as_string_decimal(&self) -> String {
		let mut hour_add = 0;
		let minute = match self.m {
			0..=7   =>  0,
			8..=22  => 25,
			23..=37 => 50,
			38..=52 => 75,
			_       =>  { hour_add = 1; 0},
		};
		
		format!["{}.{:02}", self.h + hour_add, minute]
	}
	
	pub fn difference(c1: &Clock, c2: &Clock) -> Clock {
		let s1 = Self::to_seconds(c1);
		let s2 = Self::to_seconds(c2);
		
		let sn = if s1 > s2 {
			s1 - s2
		} else {
			s2 - s1
		};
		
		Self::to_clock(sn)
	}
	
	fn h_set(h: usize) -> usize {
		if h < 24 { h } else { 0 }
	}
	
	fn m_set(m: usize) -> usize {
		if m < 60 { m } else { 0 }
	}
	
	fn s_set(s: usize) -> usize {
		if s < 60 { s } else { 0 }
	}
	
	fn is_number(s: &str) -> bool {
		match s.parse::<usize>() {
			Ok(_) => true,
			Err(_) => false,
		}
	}
	
	fn to_seconds(c: &Clock) -> usize {
		(c.h() * 60 * 60) + (c.m() * 60) + (c.s())
	}

	fn to_clock(seconds: usize) -> Clock {
		Clock::new(seconds / (60 * 60), (seconds / 60) % 60, seconds % 60)
	}
}

impl PartialEq for Clock {
	fn eq(&self, other: &Self) -> bool {
		self.h == other.h &&
		self.m == other.m &&
		self.s == other.s
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn clock_new_test() {
		let h = 14;
		let m = 03;
		let s = 56;
		
		let clock = Clock::new(h, m, s);
		
		assert!(
			clock.h() == h &&
			clock.m() == m &&
			clock.s() == s
		);
	}
	
	#[test]
	fn clock_new_invalid_test() {
		let h = 36;
		let m = 47;
		let s = 90;
		
		let clock = Clock::new(h, m, s);
		
		assert!(
			clock.h() == 0 &&
			clock.m() == m &&
			clock.s() == 0
		);
	}
	
	#[test]
	fn clock_new_from_string_test() {
		let clock = Clock::new_from_string("12:07:00").unwrap();
		
		assert!(
			clock.h() == 12 &&
			clock.m() == 7 &&
			clock.s() == 0
		);
	}
	
	#[test]
	fn clock_new_from_string_invalid_test() {
		assert_eq!(Clock::new_from_string("aBcDe"), None);
	}

	#[test]
	fn clock_as_string_test() {
		let clock = Clock::new(2, 0, 5);
		
		assert_eq!(clock.as_string(), "02:00:05");
	}

	#[test]
	fn clock_as_string_decimal_test() {
		const ENTRIES: usize = 5;
		
		let clock_list: [Clock; ENTRIES] = [
			Clock::new( 0,  2, 30),
			Clock::new( 2, 10, 12),
			Clock::new( 4, 24,  0),
			Clock::new( 8, 40, 47),
			Clock::new(10, 55, 23),
		];
		
		let string_list: [String; ENTRIES] = [
			 "0.00".to_string(),
			 "2.25".to_string(),
			 "4.50".to_string(),
			 "8.75".to_string(),
			"11.00".to_string(),
		];
		
		let mut matches = true;
		
		for i in 0..ENTRIES {
			if clock_list[i].as_string_decimal() != string_list[i] {
				matches = false;
				break;
			}
		}
		
		assert!(matches);
	}

	#[test]
	fn clock_difference_greater_first() {
		assert_eq!(
			Clock::difference(&Clock::new(14, 5, 23), &Clock::new(7, 24, 58)),
			Clock::new(6, 40, 25)
		);
	}
	
	#[test]
	fn clock_difference_greater_second() {
		assert_eq!(
			Clock::difference(&Clock::new(7, 24, 58), &Clock::new(14, 5, 23)),
			Clock::new(6, 40, 25)
		);
	}

	#[test]
	fn clock_difference_equal() {
		assert_eq!(
			Clock::difference(&Clock::new(12, 11, 10), &Clock::new(12, 11, 10)),
			Clock::new(0, 0, 0)
		);
	}

	#[test]
	fn clock_difference_one_zero() {
		assert_eq!(
			Clock::difference(&Clock::new(16, 27, 38), &Clock::new(0, 0, 0)),
			Clock::new(16, 27, 38)
		);
	}
	
	#[test]
	fn clock_difference_both_zeros() {
		assert_eq!(
			Clock::difference(&Clock::new(0, 0, 0), &Clock::new(0, 0, 0)),
			Clock::new(0, 0, 0)
		);
	}
}