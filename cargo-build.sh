#!/usr/bin/env bash

cargo build --release
cargo install --path .

rust-cli -V