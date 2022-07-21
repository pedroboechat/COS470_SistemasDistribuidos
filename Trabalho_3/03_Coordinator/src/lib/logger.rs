use std::path::PathBuf;
use std::env::current_exe;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log4rs::Handle as LogHandle;

/// Create a logger
pub fn create_logger(log_filename: &str) -> LogHandle {
    let mut log_filepath: PathBuf = current_exe().unwrap();
    log_filepath.pop();
    log_filepath.pop();
    log_filepath.pop();
    log_filepath.pop();
    log_filepath.push("log");
    log_filepath.push(log_filename);
    log_filepath.set_extension("log");

    let logfile = FileAppender::builder()
        .encoder(
            Box::new(PatternEncoder::new("{d(%s%.9f)(utc):22} - {m}{n}"))
        )
        .build(log_filepath)
        .unwrap();

    let config = Config::builder()
        .appender(
            Appender::builder().build(
                "logfile",
                Box::new(logfile)
            )
        )
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info),
        )
        .unwrap();

    return log4rs::init_config(config).unwrap();
}
