mod commands;
mod crc;
mod device;
mod logging;
mod presence;
mod util;

fn main() {
	logging::init();
	commands::run();
}
