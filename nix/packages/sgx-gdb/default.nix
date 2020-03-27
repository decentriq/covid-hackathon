# This package adds the `sgx-gdb` binary, a wrapped gdb that loads the fortanix GDB extension functions as well
# as pretty printing for Rust structures.
# Documentation of the fortanix functions: https://edp.fortanix.com/docs/tasks/debugging/
let
  pkgs = import <nixpkgs> {};
  rustTools = import ../../rust-tools.nix;
  gdb = pkgs.gdb;
  rust-sgx = pkgs.fetchFromGitHub {
    owner = "fortanix";
    repo = "rust-sgx";
    rev = "83ca347bd3d2f91ed09c4868b5a7cee225507fe8";
    sha256 = "0m5m75wq8hs388ljlssr46vwibnybd61mlz23rka46sk5n8s72pf";
  };
in pkgs.writeScriptBin "sgx-gdb" ''
  #!/usr/bin/env bash
  set -euo pipefail
  RUSTLIB_ETC="${rustTools.rustNightly.rust}/lib/rustlib/etc"
  export PYTHONPATH=${PYTHONPATH:-}:"$RUSTLIB_ETC"
  exec ${gdb}/bin/gdb \
    -iex "source ${rust-sgx}/scripts/gdb.py" \
    -iex "directory $RUSTLIB_ETC" \
    -iex "add-auto-load-safe-path $RUSTLIB_ETC/gdb_load_rust_pretty_printers.py" $@
''

