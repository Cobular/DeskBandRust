use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

pub fn configure_logging() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
        .build("stdout.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(log::LevelFilter::Debug))?;

    log4rs::init_config(config)?;

    Ok(())
}