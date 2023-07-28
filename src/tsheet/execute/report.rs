use super::data::Data;
use super::clock::Clock;

struct Report {
	date: String,
	work_order: String,
	time: f64,
}

pub fn run(data: &Vec<Data>) -> String {
	if data.len() == 0 {
		return "No data found.".to_string();
	}
	
	let mut r: Vec<Report> = Vec::new();
	
	for d in data {
		let date = d.date().clone().to_string();
		let work_order = d.work_order().clone().to_string();
		let time_difference = calculate_time_difference(d.time_out(), d.time_in());
		
		if let Some(index) = find_index(&date, &work_order, &r) {
			report_update(&mut r, index, time_difference);
		} else {
			report_add(&mut r, Report {date, work_order, time: time_difference});
		}
	}
	
	write(&r)
}

fn calculate_time_difference(c1: &Clock, c2: &Clock) -> f64 {
	Clock::difference(c1, c2).as_string_decimal().parse().unwrap()
}

fn find_index(date: &str, work_order: &str, report: &Vec<Report>) -> Option<usize> {
	let mut index = 0;
	
	for r in report {
		if r.date == date && r.work_order == work_order {
			return Some(index);
		}
		
		index += 1;
	}
	
	None
}

fn report_update(r: &mut Vec<Report>, index: usize, amount: f64) {
	r[index].time += amount;
}

fn report_add(r: &mut Vec<Report>, report_add: Report) {
	r.push(report_add);
}

fn write(r: &Vec<Report>) -> String {
	let unique_dates = get_unique_dates(r);
	let unique_work_orders = get_unique_work_orders(r);
	let mut r_string = String::new();

	for work in unique_work_orders {
		r_string.push_str(&format!["{work}\t(description)"]);
		
		for date in &unique_dates {
			if let Some(index) = find_index(&date, &work, r) {
				r_string.push_str(&format!["\t{:.02}", r[index].time]);
			} else {
				r_string.push_str(&format!["\t-"]);
			}
		}
		
		r_string.push_str("\n");
	}
	
	r_string
}

fn get_unique_dates(r: &Vec<Report>) -> Vec<String> {
	let mut v: Vec<String> = Vec::new();
	
	for q in r {
		v.push(q.date.clone());
	}
	
	v.sort();
	v.dedup();
	v
}

fn get_unique_work_orders(r: &Vec<Report>) -> Vec<String> {
	let mut v: Vec<String> = Vec::new();
	
	for q in r {
		v.push(q.work_order.clone());
	}
	
	v.sort();
	v.dedup();
	v
}