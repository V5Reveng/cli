mod commands;
mod crc;
mod device;
mod logging;
mod presence;
mod temp_dir;

fn main() {
	logging::init();
	let exit_code = commands::run();
	std::process::exit(exit_code as i32);
}
