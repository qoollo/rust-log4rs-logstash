# Logstash appender for log4rs

This crate provides appender implementation for log4rs.
Previously published under the name 'log4rs-logstash'.

# Get started

```rust
use std::time::Duration;
use qoollo_log4rs_logstash::config::DeserializersExt; 

fn main() {
    log4rs::init_file(
        "path_to_config.yaml",
        log4rs::config::Deserializers::default().with_logstash(),
    ).unwrap();
    spawn_signal_handler().unwrap();

    log::debug!("Debug");
    log::trace!("Trace");
    log::info!("Info");
    log::warn!("Warn");
    log::error!("Error");
}
```

[`examples/basic.rs`](examples/basic.rs) provides example of program with exit handling.

[`examples/basic_config.yaml`](examples/basic_config.yaml) example of config file with logstash appender.