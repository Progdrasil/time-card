use chrono::Local;

use time_card::*;

fn main() {
	let day = models::WorkDay {
		id: 1,
		start: Some(Local::now()),
		lunch_in: None,
		lunch_out: None,
		end: None,
	};

	println!("{:#?}", day);
}
