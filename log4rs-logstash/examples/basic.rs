use log4rs::init_file;

fn main() {
    init_file(
        "log4rs-logstash/examples/basic_config.yaml",
        log4rs_logstash::config::deserializers(),
    )
    .unwrap();

    log::debug!("Hello2");
}
