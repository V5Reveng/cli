mod commands;
mod logging;
mod util;

fn main() {
	logging::init();
	let exit_code = commands::run();
	std::process::exit(exit_code as i32);
}
