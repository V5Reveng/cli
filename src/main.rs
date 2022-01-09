mod commands;
mod crc;
mod device;
mod logging;
mod presence;

fn main() {
	logging::init();
	commands::run();
}
