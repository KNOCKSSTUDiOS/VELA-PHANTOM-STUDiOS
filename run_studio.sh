#!/bin/bash
export DISPLAY=:0
cargo build --release
clear
echo "=================================================="
echo "⚡ VELA PHANTOM STUDiO (NATIVE GUI)"
echo "SIG: KNOCKSSTUDiOS"
echo "=================================================="
./target/release/orchestrator &
