extern crate app_dirs;
use app_dirs::*;

const APP_INFO: AppInfo = AppInfo{ name: "Time Card", author: "Progdrasil" };

fn main() {
	println!("{:?}", get_app_root(AppDataType::UserConfig, &APP_INFO));
}
