{ callPackage, ... }:

callPackage ./generic-2.7.0-latest.nix {
  version = "2.7.0";
  url = "https://download.01.org/intel-sgx/latest/linux-latest/distro/ubuntu18.04-server/debian_pkgs/libs/libsgx-enclave-common/libsgx-enclave-common_2.7.101.3-bionic1_amd64.deb";
  sha256 = "1cdgpmb1ljrkwj487b1fcvhf3z807x81bx01r2i4d433r0rxkg22";
}
