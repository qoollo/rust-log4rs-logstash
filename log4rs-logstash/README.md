# Logstash appender for log4rs

This crate provides appender implementation for log4rs.

# Get started

```rust
use std::time::Duration;

fn main() {
    log4rs::init_file(
        "path_to_config.yaml",
        log4rs_logstash::config::deserializers(),
    ).unwrap();
    spawn_signal_handler().unwrap();

    log::debug!("Debug");
    log::trace!("Trace");
    log::info!("Info");
    log::warn!("Warn");
    log::error!("Error");
}
```

`basic.rs` example provides example of program exit handling.