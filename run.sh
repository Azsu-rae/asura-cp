#! /bin/bash

set -e

RUST_BACKTRACE=1
cargo run --bin $2
