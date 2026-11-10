#!/usr/bin/env bash
# usage: run-step.sh <logname> <crate-dir> <cmd...>   — tees cmd + output + timestamps + exit code to logs/<logname>.log
set -o pipefail
root="$(cd "$(dirname "$0")" && pwd)"
log="$root/logs/$1.log"; dir="$2"; shift 2
mkdir -p "$(dirname "$log")"
export TZ=Asia/Singapore
export RUSTUP_TOOLCHAIN=1.99.1 CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR="$root/target"
{
  echo "# start: $(date '+%F %T %Z (Asia/Singapore)')"
  echo "# cwd: $dir"; echo "# toolchain: $(rustc -V) | $(cargo -V)"
  echo "# env: RUSTUP_TOOLCHAIN=$RUSTUP_TOOLCHAIN CARGO_BUILD_JOBS=$CARGO_BUILD_JOBS"
  echo "\$ $*"
  (cd "$dir" && "$@") 2>&1; rc=$?
  echo "# exit_code: $rc"; echo "# end: $(date '+%F %T %Z (Asia/Singapore)')"
  exit $rc
} 2>&1 | tee "$log"
