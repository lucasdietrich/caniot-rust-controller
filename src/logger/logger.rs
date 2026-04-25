use std::collections::HashMap;
use std::env;

use log::LevelFilter;

const FORMAT_TARGET: bool = false;

fn syslog_process_name() -> String {
    env::current_exe()
        .ok()
        .and_then(|path| {
            path.file_name()
                .and_then(|file_name| file_name.to_str())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "caniot-controller".to_string())
}

fn parse_level() -> LevelFilter {
    match env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "debug".to_string())
        .to_lowercase()
        .as_str()
    {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Debug,
    }
}

pub fn init_logger() {
    let mut modules = HashMap::<&str, LevelFilter>::new();

    modules.insert("rocket", LevelFilter::Warn);
    modules.insert("caniot_rust_controller", LevelFilter::Debug);
    modules.insert("hyper", LevelFilter::Info);
    modules.insert("sqlx", LevelFilter::Info);

    let global_level = parse_level();

    if env::var("LOG_TO_SYSLOG").ok().as_deref() == Some("1") {
        let formatter = syslog::Formatter3164 {
            facility: syslog::Facility::LOG_USER,
            hostname: None,
            process: syslog_process_name(),
            pid: 0,
        };

        let logger = syslog::unix(formatter).expect("failed to connect to syslog");

        log::set_boxed_logger(Box::new(syslog::BasicLogger::new(logger)))
            .expect("failed to set syslog logger");

        log::set_max_level(global_level);

        // Note: syslog backend does NOT support per-module filtering,
        // so we emulate it via max level only.
    } else {
        // ---- env_logger path ----
        let mut builder = env_logger::builder();

        builder
            .format_level(true)
            .format_target(FORMAT_TARGET)
            .format_timestamp_millis()
            .filter_level(global_level);

        for (module, level) in modules {
            builder.filter_module(module, level);
        }

        // Optional: allow RUST_LOG to override everything
        if let Ok(rust_log) = env::var("RUST_LOG") {
            builder.parse_filters(&rust_log);
        }

        builder.init();
    }
}
