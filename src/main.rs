extern crate app_dirs;
extern crate chrono;
extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use app_dirs::*;
use chrono::prelude::*;
use std::error::Error;
use std::io;

const APP_INFO: AppInfo = AppInfo{ name: "Time Card", author: "Progdrasil" };

#[derive(Debug,Serialize,Deserialize)]
struct WorkDay {
	start: Option<DateTime<Local>>,
	lunch_in: Option<DateTime<Local>>,
	lunch_out: Option<DateTime<Local>>,
	end: Option<DateTime<Local>>,
}

fn main() {
	let mut day = WorkDay {
		start: Some(Local::now()),
		lunch_in: None,
		lunch_out: None,
		end: None,
	};
	println!("{:?}", day.start);
}
