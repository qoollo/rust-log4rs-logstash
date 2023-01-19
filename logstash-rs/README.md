# logstash-rs 

TCP log sender for Logstash. 
Previously published under the name 'logstash-rs'.

# Get started

Add dependency to your Cargo.toml
```toml
logstash-rs = 0.1.0
```

Or using upstream version from Github
```toml
logstash-rs = { git = "https://github.com/qoollo/rust-log4rs-logstash" }
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