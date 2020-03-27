let
  pkgs = import <nixpkgs> {};
  mozillaOverlay = pkgs.fetchFromGitHub {
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    rev = "b52a8b7de89b1fac49302cbaffd4caed4551515f";
    sha256 = "1np4fmcrg6kwlmairyacvhprqixrk7x9h89k813safnlgbgqwrqb";
  };
  rustOverlay = import "${mozillaOverlay.out}/rust-overlay.nix" pkgs pkgs;
  rustNightly = rustOverlay.rustChannelOf {
      sha256 = "125khcn07ki9waarp85g21slyd35li7rh31ppyahr22pi00y7zrj";
      date = "2019-11-13";
      channel = "nightly";
  };
  rustSgx = rustNightly.rust.override {
    targets = [ "x86_64-fortanix-unknown-sgx" ];
  };
in {
  inherit rustNightly rustSgx;
}
