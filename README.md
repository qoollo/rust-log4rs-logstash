# Logstash log sender for Rust with log4rs support

[![build](https://github.com/qoollo/rust-log4rs-logstash/actions/workflows/build.yml/badge.svg)](https://github.com/qoollo/rust-log4rs-logstash/actions/workflows/build.yml)
[![Crate Status](https://img.shields.io/crates/v/qoollo-logstash-rs?label=qoollo-logstash-rs)](https://crates.io/crates/qoollo-logstash-rs)
[![Crate Status](https://img.shields.io/crates/v/qoollo-log4rs-logstash?label=qoollo-log4rs-logstash)](https://crates.io/crates/qoollo-log4rs-logstash)

This repository contains two crates:

- [`qoollo-logstash-rs`](./logstash-rs) - LogStash log sender library for Rust.
- [`qoollo-log4rs-logstash`](./log4rs-logstash) - LogStash appender implementation for `log4rs` which uses `qoollo-logstash-rs`.

