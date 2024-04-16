use std::collections::HashMap;

use env_logger;

pub fn init_logger() {
    let mut modules = HashMap::<&str, log::LevelFilter>::new();

    modules.insert("rocket", log::LevelFilter::Warn);
    modules.insert("caniot_rust_controller", log::LevelFilter::Debug);

    let mut builder = env_logger::builder();

    builder
        .format_level(true)
        .format_target(true)
        .format_timestamp_millis()
        .filter_level(log::LevelFilter::Debug);

    for (module, level) in modules {
        builder.filter_module(module, level);
    }

    builder.init();
}
