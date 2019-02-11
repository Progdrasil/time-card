use std::path::PathBuf;
use clap::{App, Arg, ArgMatches, SubCommand};

// enum Opt {
// 	/// clock in an action
// 	#[structopt(name = "in")]
// 	In {
// 		/// Add a note to the action
// 		#[structopt(name = "note")]
// 		note: String,

// 		/// Use this time instead of the current time
// 		#[structopt(long)]
// 		at: chrono::NaiveTime,
// 	},

// 	/// Clock out of an action
// 	#[structopt(name = "out")]
// 	Out {
// 		/// Use this time instead of the current time
// 		#[structopt(long)]
// 		at: chrono::NaiveTime,
// 	},

// 	/// Set or get the current Task
// 	#[structopt(name = "task")]
// 	Task {
// 		/// Switch to a task list, creating it if necessary
// 		#[structopt(name = "name")]
// 		name: String,
// 	},

// 	/// Display the actions of the current day, or a specific set
// 	#[structopt(name = "display")]
// 	Display {
// 		/// Display the action ID's
// 		#[structopt(short = "i", long = "ids")]
// 		ids: bool,

// 		/// Display a specific Task
// 		#[structopt(short = "t", long = "task")]
// 		task: String,

// 		/// Display a list of days starting from a specific date
// 		#[structopt(short = "s", long = "start")]
// 		start: String,

// 		/// Display a list of days ending at the specific date
// 		#[structopt(short = "e", long = "end")]
// 		end: String,

// 		/// Display with a specific format, to get list of formats run "config --formats"
// 		#[structopt(short = "f", long = "format")]
// 		format: String,
// 	},

// 	/// Configure the application
// 	#[structopt(name = "config")]
// 	Config {
// 		/// Set the location of the configuration file, or display it
// 		#[structopt(long, parse(from_os_str))]
// 		location: PathBuf,

// 		/// Get the list of available display formats
// 		#[structopt(long)]
// 		formats: bool,

// 		/// Set the defaut format (can be a list)
// 		#[structopt(long)]
// 		format: String,
// 	}

// }

fn main() {
	let matches = commands();

	println!("{:#?}", matches.usage());
}

fn commands<'a>() -> ArgMatches<'a> {
	App::new("TimeCard")
		.version("0.1.0")
		.author("Rene Leveille <reneb.leveille@gmail.com")
		.about("A time management cli application, similar to timetrap")
		.alias("tc")
		.subcommand(SubCommand::with_name("in")
			.about("clock in an action")
			.alias("i")
			.arg(Arg::with_name("note")
				.help("Add a note to the action")
				.value_name("NOTE"))
			.arg(Arg::with_name("at")
				.help("Use this time instead of the current time")
				.long("at")
				.takes_value(true)
				.value_name("TIME")))
		.subcommand(SubCommand::with_name("out")
			.about("Clock out of an action")
			.alias("o")
			.arg(Arg::with_name("at")
				.help("Use this time instead of the current time")
				.long("at")
				.takes_value(true)
				.value_name("TIME")))
		.subcommand(SubCommand::with_name("task")
			.about("Set or get the current Task")
			.alias("t")
			.arg(Arg::with_name("name")
				.help("Switch to task with this name, creating it if necessary")
				.required(true)
				.value_name("NAME")))
		.subcommand(SubCommand::with_name("project")
			.about("Set or get the current project")
			.alias("p")
			.arg(Arg::with_name("name")
				.help("Switch to project with this name, creating it if necessary")
				.required(true)
				.value_name("NAME")))
		.subcommand(SubCommand::with_name("display")
			.about("Display the actions of the current day, or a specific set")
			.alias("d")
			.arg(Arg::with_name("ids")
				.help("Display the action ID's")
				.long("ids")
				.short("i")
				.takes_value(false))
			.arg(Arg::with_name("task")
				.help("Display a specific task")
				.long("task")
				.short("t")
				.takes_value(true)
				.value_name("TASK NAME"))
			.arg(Arg::with_name("start")
				.help("Display a list of days starting from a specific date")
				.long("start")
				.short("s")
				.takes_value(true)
				.value_name("DATE"))
			.arg(Arg::with_name("end")
				.help("Display a list of days ending at the specific date")
				.long("end")
				.short("e")
				.takes_value(true)
				.value_name("DATE"))
			.arg(Arg::with_name("format")
				.help("Display with a specific format, to get list of formats run \"config --formats\"")
				.long("format")
				.short("f")
				.takes_value(true)
				.value_name("FORMAT")))
		.subcommand(SubCommand::with_name("config")
			.about("Configure the application")
			.alias("c")
			.arg(Arg::with_name("location")
				.help("Set the location of the configuration file, or display it")
				.value_name("FILE LOCATION"))
			.arg(Arg::with_name("formats")
				.help("Get the list of available display formats")
				.long("formats")
				.takes_value(false))
			.arg(Arg::with_name("format")
				.help("Set the defaut format (can be called multiple times)")
				.long("format")
				.short("f")
				.takes_value(true)
				.multiple(true)
				.value_name("FORMAT NAME")))
		.get_matches()
}

// 	/// Configure the application
// 	#[structopt(name = "config")]
// 	Config {
// 		/// Set the location of the configuration file, or display it
// 		#[structopt(long, parse(from_os_str))]
// 		location: PathBuf,

// 		/// Get the list of available display formats
// 		#[structopt(long)]
// 		formats: bool,

// 		/// Set the defaut format (can be a list)
// 		#[structopt(long)]
// 		format: String,
// 	}