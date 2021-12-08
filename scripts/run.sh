#!/usr/bin/env bash

(
    cd experiment
    echo "Build Node Template..."
    cargo build

    echo "Run Node..."
    RUST_BACKTRACE=1 ./target/debug/rpc-node --dev --validator --execution=Native --no-telemetry --no-prometheus --rpc-methods=Unsafe --rpc-cors=all
)