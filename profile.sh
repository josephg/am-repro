#!/usr/bin/env bash
set -e

RUSTFLAGS="-Cforce-frame-pointers=yes" cargo build --profile profiling
#perf record -g -F 9999 --call-graph fp target/profiling/examples/profile
perf record -g -F 99 --call-graph fp target/profiling/am-repro
perf script -F +pid > /tmp/test.perf

echo "Perf data in perf.data and script in /tmp/test.perf"
