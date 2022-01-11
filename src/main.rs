mod commands;
mod crc;
mod device;
mod logging;
mod presence;
mod temp_dir;

fn main() {
	logging::init();
	commands::run();
}
