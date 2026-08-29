#!/bin/bash
cd "$(dirname "$0")"
cargo build --release
nohup ./target/release/orchestrator > orchestrator.log 2>&1 &
echo "=================================================="
echo "⚡ VELA PHANTOM STUDiO IS LIVE"
echo "SIG: KNOCKSSTUDiOS"
echo "PID: $!"
echo "LOGS: orchestrator.log"
echo "=================================================="
