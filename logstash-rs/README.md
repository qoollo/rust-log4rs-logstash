# Logstash log sender

[![Crate Status](https://img.shields.io/crates/d/qoollo-logstash-rs)](https://crates.io/crates/qoollo-logstash-rs)

TCP log sender for Logstash. 

Previously published under the name [`logstash-rs`](https://crates.io/crates/logstash-rs).

# Get started

Add dependency to your Cargo.toml
```
qoollo-logstash-rs = 0.2
```

Or using upstream version from Github
```toml
qoollo-logstash-rs = { git = "https://github.com/qoollo/rust-log4rs-logstash" }
```

Initialize logger
```rust
use qoollo_logstash_rs::{BufferedSender, TcpSender};
use std::time::Duration;

fn main() {
    let logger = BufferedSender::new(
        TcpSender::new("localhost".to_string(), 3055, false), // hostname, port, use tls
        Some(64), // buffer size
        Some(Duration::from_secs(60)), // buffer lifetime
    );
    log::set_boxed_logger(Box::new(logger)).unwrap();

    log::error!("Test");
}
```