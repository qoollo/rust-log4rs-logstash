#[macro_use]
extern crate log;

use std::time::Duration;

use anyhow::Result as AnyResult;
use log4rs::init_file;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
    low_level::exit,
};

fn main() -> AnyResult<()> {
    init_logger()?;
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

fn init_logger() -> AnyResult<()> {
    init_file(
        "log4rs-logstash/examples/basic_config.yaml",
        log4rs_logstash::config::deserializers(),
    )?;
    Ok(())
}

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
