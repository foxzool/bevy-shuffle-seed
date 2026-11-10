#!/usr/bin/env bash
# Runs E5 binary (bevy_ecs 0.20.1 + debug, release build). One process per run.
B="$(cd "$(dirname "$0")" && pwd)/target/release/e5-ecs-debug-fixed"
set -e
echo "## repeat: 5 processes per seed"
for s in none 1 7 42 100000001 100000030; do
  for i in 1 2 3 4 5; do echo "### run=$i"; "$B" "$s"; done
done
echo "## scan: seeds 0..63 + official test seeds"
for s in none $(seq 0 63) 100000000 100000001 100000003 100000004 100000020 100000030 100000063 100000080; do "$B" "$s"; done
