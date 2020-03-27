#!/usr/bin/env bash
set -euo pipefail

maturin build --release \
  --manifest-path ../Cargo.toml \
  --cargo-extra-args='--features python' \
  --manylinux 2010 \
  --out pkg/
