use anyhow::Result as AnyResult;
use log4rs::init_file;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
    low_level::exit,
};

fn main() {
    try_main().unwrap();
}

fn try_main() -> AnyResult<()> {
    init_logger()?;
    spawn_signal_handler()?;

    log::logger().flush();
    Ok(())
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
        for _sig in signals.forever() {
            log::logger().flush();
            exit(0);
        }
    });
    Ok(())
}
