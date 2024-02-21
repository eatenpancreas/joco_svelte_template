#!/bin/bash
cargo sqlx migrate run
cargo sqlx prepare --workspace
cargo build --release
./target/release/api
