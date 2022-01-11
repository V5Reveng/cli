use clap::lazy_static::lazy_static;
use log::{Level, LevelFilter, Log, Metadata, Record};
use phf::phf_map;
use std::collections::HashMap;
use std::sync::RwLock;

static LEVELS: phf::Map<&'static str, LevelFilter> = phf_map! {
	"trace" => LevelFilter::Trace,
	"debug" => LevelFilter::Debug,
	"info" => LevelFilter::Info,
	"warn" => LevelFilter::Warn,
	"error" => LevelFilter::Error,
	"off" => LevelFilter::Off,
};
static INVALID_LEVEL: &'static str = "Invalid log level provided via REVENG_LOG_LEVEL; valid levels are trace, debug, info, warn, error, off";

static mut BASE_TIMESTAMP: Option<std::time::Instant> = None;
static mut GLOBAL_LEVEL: LevelFilter = if cfg!(debug_assertions) { LevelFilter::Debug } else { LevelFilter::Info };
lazy_static! {
	static ref LOCAL_LEVELS: RwLock<HashMap<String, LevelFilter>> = Default::default();
}
static LOGGER: SimpleLogger = SimpleLogger(());

/// On error, returns a reason along with the invalid string
fn parse_and_set(raw: &str) -> Result<(), (&'static str, &str)> {
	for item in raw.split(',') {
		match item.rsplit_once('=') {
			None => {
				let level = item;
				let level = LEVELS.get(level).ok_or((INVALID_LEVEL, level))?;
				self::set_level(*level);
			}
			Some((module, level)) => {
				let level = LEVELS.get(level).ok_or((INVALID_LEVEL, level))?;
				LOCAL_LEVELS.write().unwrap().insert(module.to_owned(), *level);
			}
		}
	}
	Ok(())
}

pub fn init() {
	unsafe {
		BASE_TIMESTAMP = Some(std::time::Instant::now());
	}
	log::set_logger(&LOGGER).expect("Could not set logger");
	// do our own filtering
	log::set_max_level(LevelFilter::Trace);
	match std::env::var("REVENG_LOG_LEVEL") {
		Ok(env_level) => parse_and_set(&env_level).expect("Could not parse REVENG_LOG_LEVEL environment variable"),
		Err(std::env::VarError::NotUnicode(_)) => panic!("REVENG_LOG_LEVEL environment variable is not valid Unicode"),
		Err(_) => (),
	}
}

pub fn level() -> LevelFilter {
	unsafe { GLOBAL_LEVEL }
}
pub fn set_level(level: LevelFilter) {
	LOCAL_LEVELS.write().unwrap().clear();
	unsafe {
		GLOBAL_LEVEL = level;
	}
}
pub fn set_from_int(level: usize) {
	set_level(match level {
		0 => LevelFilter::Info,
		1 => LevelFilter::Debug,
		_ /* 2.. */ => LevelFilter::Trace,
	});
}

static CRATE_PREFIX: &'static str = "reveng_cli::";

/// The absolute most simple logger
/// Logs to stderr and has no local level
struct SimpleLogger(());
impl SimpleLogger {
	fn actually_enabled(level: Level, module_path: Option<&str>) -> bool {
		let max_level = module_path.and_then(|module_path| LOCAL_LEVELS.read().unwrap().get(module_path).copied()).unwrap_or(self::level());
		level <= max_level
	}
}
impl Log for SimpleLogger {
	fn enabled(&self, metadata: &Metadata<'_>) -> bool {
		metadata.level() <= level()
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
