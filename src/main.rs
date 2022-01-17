//! The Reveng CLI allows for interfacing with the VEX V5 Cortex.
//!
//! Unlike alternatives, it is actually open source, with no files excluded.

mod commands;
mod crc;
mod device;
mod logging;
mod program;
mod util;

fn main() {
	logging::init();
	let exit_code = commands::run();
	std::process::exit(exit_code as i32);
}
