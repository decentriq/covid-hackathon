let
  pkgs = import <nixpkgs> {};
  rustTools = import ./rust-tools.nix;
  nixpkgsPin = builtins.readFile ./NIXPKGS_PIN;
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustTools.rustSgx
    rustTools.rustNightly.cargo
  ];
  buildInputs = with pkgs; [
    (import packages/elf2sgxs)
    (import packages/runner-cargo)
    (import packages/sgx-gdb)
    go
    pkgconfig
    openssl
    cmake
    cacert
    nix
    llvmPackages.llvm

    automake
    libtool
  ];

  hardeningDisable = [ "fortify" ];
  RUST_BACKTRACE = "full";
  RUST_LOG = "info";

  shellHook = ''
    export LIBCLANG_PATH=${pkgs.llvmPackages.clang-unwrapped.lib}/lib
  '';

  NIX_PATH = "nixpkgs=${nixpkgsPin}:nixos=${nixpkgsPin}";
}
