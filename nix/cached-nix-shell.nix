let
  pkgs = import <nixpkgs> {};
  cached-nix-shell-src = builtins.fetchTarball {
    url = "https://github.com/xzfc/cached-nix-shell/archive/v0.1.1.tar.gz";
    sha256 = "0j39mqhvjrqkcx4yfaxlak4832bx9n2w6a5j0gfnq1a8spp5fj8a";
  };
in pkgs.mkShell {
  buildInputs = [
    (import "${cached-nix-shell-src}" {})
    pkgs.nix
  ];
}
