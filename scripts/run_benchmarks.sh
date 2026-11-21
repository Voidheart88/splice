#!/usr/bin/env bash
set -e

# Log file for benchmark output
LOG_FILE="benchmark_results.log"

echo "Running benchmarks..." | tee "$LOG_FILE"
# Run cargo benchmark and capture both stdout and stderr
cargo bench 2>&1 | tee -a "$LOG_FILE"

echo "Benchmarks completed. Detailed Criterion reports are in ./target/criterion"

echo "Full log saved to $LOG_FILE"
