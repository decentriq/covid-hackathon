{ config, lib, pkgs, ... }:

with lib;

let

  intel = config.hardware.cpu.intel;

in {

  ###### interface

  options = {
    hardware.cpu.intel.sgx = {
      enable = mkOption {
        default = false;
        type = types.bool;
        description = ''
          Enable SGX.
          This will allow SGX enclaves to be loaded, assuming SGX is enabled in the BIOS.
          Note: loading this driver taints the kernel
        '';
      };

      package = mkOption {
        type = types.package;
        description = ''
          SGX driver to use.
        '';
      };
    };

    hardware.cpu.intel.aesmd = {
      enable = mkOption {
        type = types.bool;
        default = false;
        description = ''
          Enable Intel's Architectural Enclave Service Manager for Intel SGX
        '';
      };

      package = mkOption {
        type = types.package;
        default = pkgs.intel-aesmd;
        defaultText = "pkgs.intel-aesmd";
        description = ''
          aesmd derivation to use.
        '';
      };
    };
  };


  ###### implementation

  config = mkMerge [
    (mkIf (intel.sgx.enable) {
      boot.kernelModules = [ "isgx" ];
      boot.extraModulePackages = [ intel.sgx.package ];
    })

    (mkIf intel.aesmd.enable {
      users.groups.aesmd = {
      };

      users.users.aesmd = {
        description = "aesmd user";
        isSystemUser = true;
        group = "aesmd";
      };

      environment.etc."aesmd.conf" = {
        text = "";
      };

      systemd.packages = [ intel.aesmd.package ];
      systemd.services.aesmd.wantedBy = [ "multi-user.target" ];
      systemd.services.aesmd.serviceConfig = {
        User = "aesmd";
      };
      hardware.cpu.intel.sgx.enable = true;
    })
  ];

}
