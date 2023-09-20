use env_logger;

pub fn init_logger() {
    env_logger::builder()
    .format_level(true)
    .format_target(true)
    .format_timestamp_millis()
    .filter_level(log::LevelFilter::Debug)
    .init();
}