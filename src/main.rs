mod commands;
mod logging;

fn main() {
	logging::init();
	commands::run();
}
