use std::collections::HashMap;

use env_logger;

// struct Config {
//     filepath: String,
//     modules_level: HashMap<String, log::LevelFilter>,
// }

pub fn init_logger() {
    let mut modules = HashMap::<&str, log::LevelFilter>::new();

    modules.insert("rocket", log::LevelFilter::Warn);
    modules.insert("caniot_rust_controller", log::LevelFilter::Debug);
    modules.insert("hyper", log::LevelFilter::Warn);

    let mut builder = env_logger::builder();

    builder
        .format_level(true)
        .format_target(false)
        // .format_module_path(false)
        .format_timestamp_millis()
        .filter_level(log::LevelFilter::Debug);

    for (module, level) in modules {
        builder.filter_module(module, level);
    }

    builder.init();
}
