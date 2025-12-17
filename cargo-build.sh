#!/usr/bin/env bash

cargo build --release
cargo doc
cargo install --path .

rust-cli -V