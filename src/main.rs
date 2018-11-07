// extern crate app_dirs;
extern crate chrono;
// extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate serde;

// use app_dirs::*;
use chrono::prelude::*;
// use std::error::Error;
// use std::io;

// const APP_INFO: AppInfo = AppInfo{ name: "Time Card", author: "Progdrasil" };

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

mod simpledb {
	use std::collections::HashMap;
	use std::error::Error;
	use std::sync::{Arc, RwLock};

	type KVCollection = HashMap<Vec<u8>, Vec<u8>>;
	type Records = Arc<RwLock<KVCollection>>;

	pub struct SimpleDB {
		records: Records,
	}

	pub fn open() {}

	fn create() -> Result<SimpleDB, Error> {
		let db = SimpleDB {
			records: Arc::new(RwLock::new(KVCollection::new()))
		};
		Ok(db)
	}

	unsafe impl Sync for SimpleDB {}
	unsafe impl Send for SimpleDB {}
}
