use super::{BASE_TIMESTAMP, LOCAL_LEVELS};
use log::{Level, Log, Metadata, Record};
use std::io::Write;

/// The absolute most simple logger.
/// Logs to stderr and has no local level.
pub struct SimpleLogger(());

impl SimpleLogger {
	/// `Log::enabled` doesn't offer enough information, so we use this instead.
	fn actually_enabled(level: Level, module_path: Option<&str>) -> bool {
		let max_level = module_path.and_then(|module_path| LOCAL_LEVELS.read().unwrap().get(module_path).copied()).unwrap_or_else(super::level);
		level <= max_level
	}
	pub const fn new() -> Self {
		Self(())
	}
}

impl Log for SimpleLogger {
	fn enabled(&self, metadata: &Metadata<'_>) -> bool {
		metadata.level() <= super::level()
	}
	fn log(&self, record: &Record<'_>) {
		if !Self::actually_enabled(record.level(), record.module_path()) {
			return;
		}
		let time = unsafe { BASE_TIMESTAMP }.expect("Logging has not been properly initialized").elapsed();
		eprintln!(
			"[{time:0>7.3}][{level: >5}][{path: <16}]  {content}",
			time = time.as_secs_f64(),
			level = ColorizedLevel(record.level()),
			path = record.module_path().unwrap_or(""),
			content = record.args()
		);
	}
	fn flush(&self) {
		let _ = std::io::stderr().flush();
	}
}

struct ColorizedLevel(Level);
impl std::fmt::Display for ColorizedLevel {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		use colored::Colorize;
		let s = match self.0 {
			Level::Trace => "trace".white(),
			Level::Debug => "debug".normal(),
			Level::Info => "info".blue().bold(),
			Level::Warn => "warn".yellow().bold(),
			Level::Error => "error".red().bold(),
		};
		write!(formatter, "{}", s)
	}
}
