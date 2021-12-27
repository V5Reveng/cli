pub fn init() {
	let env = env_logger::Env::new().filter_or("REVENG_LOG_LEVEL", "info").write_style_or("REVENG_LOG_STYLE", "auto");
	env_logger::init_from_env(env);
}

pub fn set_from_int(level: u32) {
	log::set_max_level(match level {
		0 => log::LevelFilter::Info,
		1 => log::LevelFilter::Debug,
		// 2 or more
		_ => log::LevelFilter::Trace,
	});
}
