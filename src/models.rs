use chrono::prelude::{DateTime, Local};
use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
	pub id: i32,
	pub start: Option<DateTime<Local>>,
	pub end: Option<DateTime<Local>>,
	pub note: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
	pub name: String,
	pub actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Break {
	pub start: Option<DateTime<Local>>,
	pub end: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkDay {
	pub id: i32,
	pub start: Option<DateTime<Local>>,
	pub end: Option<DateTime<Local>>,
	pub lunch: Option<Break>,
	pub breaks: Vec<Break>,
	pub tasks: Vec<Task>
}
