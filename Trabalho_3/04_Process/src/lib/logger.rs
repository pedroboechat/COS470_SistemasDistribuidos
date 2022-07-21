use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log4rs::Handle as LogHandle;

/// Create a logger
pub fn create_logger(log_filename: &str) -> LogHandle {
    let logfile = FileAppender::builder()
        .encoder(
            Box::new(PatternEncoder::new("{d(%s%.9f)(utc):22} - {m}{n}"))
        )
        .build(format!("../log/{}.txt", log_filename))
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
