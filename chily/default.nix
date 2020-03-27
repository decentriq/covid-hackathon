let
  pkgs = import <nixpkgs> {};
  rustTools = import ../nix/rust-tools.nix;
in pkgs.rustPlatform.buildRustPackage rec {
  name = "chily";
  src = ./.;
  cargoSha256 = "0rcycbvyjsn09kn4c6rlad99sxcqk5h9mgny46paly2f4fgqs9x3";

  nativeBuildInputs = with pkgs; [
    rustTools.rustSgx
    rustTools.rustNightly.cargo
  ];
  buildInputs = with pkgs; [
    go
    pkgconfig
    openssl
    cmake
    protobuf
    perl
    cacert
  ];

  # TODO switch off nix's auto-fortification, which breaks debug builds. See https://github.com/NixOS/nixpkgs/issues/60919
  hardeningDisable = [ "fortify" ];
  RUST_BACKTRACE = "full";
  RUST_LOG = "info";
}
