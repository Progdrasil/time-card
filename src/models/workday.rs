use chrono::prelude::{DateTime, Local};
use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct WorkDay {
	pub id: i32,
	pub start: Option<DateTime<Local>>,
	pub lunch_in: Option<DateTime<Local>>,
	pub lunch_out: Option<DateTime<Local>>,
	pub end: Option<DateTime<Local>>,
}
