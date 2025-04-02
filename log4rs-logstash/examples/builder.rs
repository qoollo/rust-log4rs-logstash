#[macro_use]
extern crate log;

use std::time::Duration;

use anyhow::Result as AnyResult;
use log::LevelFilter;
use log4rs::{
    config::{Appender, Root},
    init_config, Config,
};

#[cfg(not(windows))]
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
    low_level::exit,
};

fn main() -> AnyResult<()> {
    let logstash = qoollo_log4rs_logstash::appender::AppenderBuilder::default()
        .with_hostname("my-hostname")
        .with_port(5000)
        .with_buffer_size(100)
        .with_buffer_lifetime(Duration::from_secs(1))
        .build()
        .unwrap();
    let appender = Appender::builder().build("logstash", Box::new(logstash));
    let config = Config::builder().appender(appender).build(
        Root::builder()
            .appender("logstash")
            .build(LevelFilter::Warn),
    )?;
    init_config(config)?;

    #[cfg(not(windows))]
    spawn_signal_handler()?;

    loop {
        std::thread::sleep(Duration::from_secs(1));
        debug!("Debug");
        trace!("Trace");
        info!("Info");
        warn!("Warn");
        error!("Error");
    }
}

#[cfg(not(windows))]
fn spawn_signal_handler() -> AnyResult<()> {
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?;

    std::thread::spawn(move || {
        let mut stop_in_progress = false;
        for _sig in signals.forever() {
            std::thread::spawn(move || {
                log::logger().flush();
                exit(0)
            });
            if stop_in_progress {
                exit(1);
            }
            stop_in_progress = true;
        }
    });
    Ok(())
}
