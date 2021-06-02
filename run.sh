#!/bin/bash
until RUST_LOG=info cargo run --release; do
    echo "Test suite crashed with exit code $?.  Respawning.." >&2
    sleep 1
done