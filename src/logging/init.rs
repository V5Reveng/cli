use super::{BASE_TIMESTAMP, LOCAL_LEVELS, LOGGER};
use log::LevelFilter;
use phf::phf_map;
use std::time::Instant;
use std::{env, panic};

static LEVELS: phf::Map<&'static str, LevelFilter> = phf_map! {
	"trace" => LevelFilter::Trace,
	"debug" => LevelFilter::Debug,
	"info" => LevelFilter::Info,
	"warn" => LevelFilter::Warn,
	"error" => LevelFilter::Error,
	"off" => LevelFilter::Off,
};
static INVALID_LEVEL: &str = "Invalid log level provided via REVENG_LOG_LEVEL; valid levels are trace, debug, info, warn, error, off";

/// On error, returns a reason along with the invalid string
fn parse_and_set(raw: &str) -> Result<(), (&'static str, &str)> {
	for item in raw.split(',') {
		match item.rsplit_once('=') {
			None => {
				let level = item;
				let level = LEVELS.get(level).ok_or((INVALID_LEVEL, level))?;
				super::set_level(*level);
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
		BASE_TIMESTAMP = Some(Instant::now());
	}
	log::set_logger(&LOGGER).expect("Could not set logger");
	// do our own filtering
	log::set_max_level(LevelFilter::Trace);
	match env::var("REVENG_LOG_LEVEL") {
		Ok(env_level) => parse_and_set(&env_level).expect("Could not parse REVENG_LOG_LEVEL environment variable"),
		Err(env::VarError::NotUnicode(_)) => panic!("REVENG_LOG_LEVEL environment variable is not valid Unicode"),
		Err(_) => (),
	}
	panic::set_hook(Box::new(move |info: &panic::PanicInfo| {
		use log::{debug, error};
		let info = format!("{}", info);
		// if possible, we want to convert from "panicked at 'text'" to just "text"
		if let Some((message, location)) = info.rsplit_once(' ') {
			debug!("panic occurred at {}", location);
			if let Some(shorter_message) = message.strip_prefix("panicked at ").and_then(|x| x.strip_suffix(',')) {
				error!("{}", shorter_message.trim_matches('\''))
			} else {
				error!("{}", message);
			}
		} else {
			error!("{}", info);
		}
	}));
}
