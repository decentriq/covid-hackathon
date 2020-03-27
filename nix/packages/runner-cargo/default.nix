let
  pkgs = import <nixpkgs> {};
  rustTools = import ../../rust-tools.nix;
in pkgs.rustPlatform.buildRustPackage rec {
  name = "runner-cargo";
  version = "0.7.0";

  src = pkgs.fetchFromGitHub {
    owner = "fortanix";
    repo = "rust-sgx";
    rev = "83ca347bd3d2f91ed09c4868b5a7cee225507fe8";
    sha256 = "0m5m75wq8hs388ljlssr46vwibnybd61mlz23rka46sk5n8s72pf";
  };
  cargoSha256 = "0bbasg4b6gp3bsj0dyg2b4y84d2j2w19hs7xn3zg2mk21svvg6z4";

  nativeBuildInputs = [ rustTools.rustNightly.rust rustTools.rustNightly.cargo ];
  buildInputs = with pkgs; [ cacert git pkgconfig openssl protobuf ];

  buildPhase = ''
    cargo build --package fortanix-sgx-tools --bin ftxsgx-runner-cargo --release
    cargo build --package fortanix-sgx-tools --bin ftxsgx-runner --release
  '';
  checkPhase = ""; # cargo test --package fortanix-sgx-tools --release
  installPhase = ''
    install -vD target/release/ftxsgx-runner-cargo $out/bin/ftxsgx-runner-cargo
    install -vD target/release/ftxsgx-runner $out/bin/ftxsgx-runner
  '';

  meta = with pkgs.lib; {
    description = "Installs ftxsgx-runner-cargo, which converts a 64bit ELF SGX enclave into an SGXS enclave";
    homepage = https://github.com/fortanix/rust-sgx;
    license = licenses.mpl20;
    maintainers = [ maintainers.exfalso ];
    platforms = platforms.linux;
  };
}
