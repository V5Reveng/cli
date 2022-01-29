mod commands;
mod logging;
mod util;

fn main() -> anyhow::Result<()> {
	logging::init();
	commands::run()
}
