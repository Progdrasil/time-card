use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "TimeCard", about = "A time management cli application, similar to timetrap", version = "0.1.0", author = "Rene Leveille")]
enum Opt {
	/// clock in an action
	#[structopt(name = "in")]
	In {
		/// Add a note to the action
		#[structopt(name = "note")]
		note: String,

		/// Use this time instead of the current time
		#[structopt(long)]
		at: chrono::NaiveTime,
	},

	/// Clock out of an action
	#[structopt(name = "out")]
	Out {
		/// Use this time instead of the current time
		#[structopt(long)]
		at: chrono::NaiveTime,
	},

	/// Set or get the current Task
	#[structopt(name = "task")]
	Task {
		/// Switch to a task list, creating it if necessary
		#[structopt(name = "name")]
		name: String,
	},

	/// Display the actions of the current day, or a specific set
	#[structopt(name = "display")]
	Display {
		/// Display the action ID's
		#[structopt(short = "i", long = "ids")]
		ids: bool,

		/// Display a specific Task
		#[structopt(short = "t", long = "task")]
		task: String,

		/// Display a list of days starting from a specific date
		#[structopt(short = "s", long = "start")]
		start: String,

		/// Display a list of days ending at the specific date
		#[structopt(short = "e", long = "end")]
		end: String,

		/// Display with a specific format, to get list of formats run "config --formats"
		#[structopt(short = "f", long = "format")]
		format: String,
	},

	/// Configure the application
	#[structopt(name = "config")]
	Config {
		/// Set the location of the configuration file, or display it
		#[structopt(long, parse(from_os_str))]
		location: PathBuf,

		/// Get the list of available display formats
		#[structopt(long)]
		formats: bool,

		/// Set the defaut format (can be a list)
		#[structopt(long)]
		format: String,
	}

}

fn main() {
	let opt = Opt::from_args();

	println!("{:#?}", opt);
}
