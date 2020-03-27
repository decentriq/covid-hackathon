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
      sha256 = "1w75wp7kfafvldr49d64vzrxxll2dglbsm4j28a9f9yxc12dgn14";
      date = "2020-03-18";
      channel = "nightly";
  };
  rustSgx = rustNightly.rust.override {
    targets = [ "x86_64-fortanix-unknown-sgx" ];
  };
in {
  inherit rustNightly rustSgx;
}
