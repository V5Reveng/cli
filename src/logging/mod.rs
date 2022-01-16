use clap::lazy_static::lazy_static;
use log::LevelFilter;
use std::collections::HashMap;
use std::sync::RwLock;

mod init;
mod logger;

pub use init::init;
use logger::SimpleLogger;

static mut BASE_TIMESTAMP: Option<std::time::Instant> = None;
static mut GLOBAL_LEVEL: LevelFilter = LevelFilter::Info;

lazy_static! {
	static ref LOCAL_LEVELS: RwLock<HashMap<String, LevelFilter>> = Default::default();
}

static LOGGER: SimpleLogger = SimpleLogger::new();

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
