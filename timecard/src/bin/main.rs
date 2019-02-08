#[macro_use]
extern crate structopt;

use chrono::Local;
use structopt::StructOpt;

use timecard::models;

#[derive(Debug, StructOpt)]
#[structopt(name = "TimeCard", about = "A time management cli application, similar to timetrap", version = "0.1.0", author = "Rene Leveille")]
enum Opt {
	/// clock in an action
	#[structopt(name = "in")]
	In {
		#[structopt(name = "note")]
		note: String,
	}

}

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

	let opt = Opt::from_args();

	println!("{:#?}", opt);
}
