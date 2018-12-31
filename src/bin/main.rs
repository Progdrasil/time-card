use chrono::Local;

use time_card::*;

fn main() {
	let action = models::Action {
		id: 1,
		start: Some(Local::now()),
		end: None,
		note: String::from("this is a test action"),
	};

	let task = models::Task {
		name: String::from("Dev"),
		actions: vec![action],
	};

	let lunch = Some(models::Break {
		start: Some(Local::now()),
		end: None
	});

	let day = models::WorkDay {
		id: 1,
		start: Some(Local::now()),
		lunch,
		end: None,
		breaks: Vec::new(),
		tasks: vec![task],
	};

	println!("{:#?}", day);
}
