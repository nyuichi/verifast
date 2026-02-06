#!/usr/bin/env bash

set -euo pipefail

script_dir="$(cd "$(dirname "$0")" && pwd)"
test_file="$script_dir/exporter_duplicate_body.rs"
exporter="$script_dir/../../../bin/vf-rust-mir-exporter"
tmp_out="$(mktemp)"
tmp_err="$(mktemp)"
trap 'rm -f "$tmp_out" "$tmp_err"' EXIT

sysroot="$(rustc --print sysroot)"
host="$(rustc -vV | sed -n 's/^host: //p')"
rustc_lib="$sysroot/lib/rustlib/$host/lib"
export DYLD_LIBRARY_PATH="$sysroot/lib:$rustc_lib:${DYLD_LIBRARY_PATH:-}"
export LD_LIBRARY_PATH="$sysroot/lib:$rustc_lib:${LD_LIBRARY_PATH:-}"

"$exporter" "$test_file" \
  --vf-rust-mir-exporter:no-default-args \
  --crate-type=lib \
  --error-format=json \
  >"$tmp_out" 2>"$tmp_err"

grep -q "Skipping duplicate body for outer::inner" "$tmp_err"
