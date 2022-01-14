use super::{BASE_TIMESTAMP, LOCAL_LEVELS};
use log::{Level, Log, Metadata, Record};

static CRATE_PREFIX: &str = "reveng_cli::";

/// The absolute most simple logger
/// Logs to stderr and has no local level
pub struct SimpleLogger(());
impl SimpleLogger {
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
		if !Self::actually_enabled(record.level(), record.module_path().and_then(|x| x.strip_prefix(CRATE_PREFIX))) {
			return;
		}
		let time = unsafe { BASE_TIMESTAMP }.expect("Logging has not been properly initialized").elapsed();
		eprintln!(
			"[{time:0>7.3}][{level: >5}][{path: <8}]  {content}",
			time = time.as_secs_f64(),
			level = record.level(),
			path = record.module_path().map(|path| path.strip_prefix(CRATE_PREFIX).unwrap_or(path)).unwrap_or(""),
			content = record.args()
		);
	}
	fn flush(&self) {
		let _ = std::io::Write::flush(&mut std::io::stderr());
	}
}
