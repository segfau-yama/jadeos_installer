{ config, lib, ... }:
let
  installArgsPath = ./install-args.nix;
  installArgs =
    if builtins.pathExists installArgsPath
    then import installArgsPath
    else { };
  installDisk = installArgs.installDisk or config.my.installDisk;
  generatedHardwareModule =
__JADE_GENERATED_HARDWARE_MODULE__
in
{
  imports = [
    generatedHardwareModule
  ];

  fileSystems."/" = lib.mkForce {
    device = installDisk.root;
    fsType = "ext4";
  };

  fileSystems."/boot" = lib.mkForce {
    device = installDisk.boot;
    fsType = "vfat";
  };

  swapDevices = lib.mkForce (
    lib.optional (installDisk.swap != null && installDisk.swap != "") {
      device = installDisk.swap;
    }
  );

  networking.useDHCP = lib.mkDefault true;

  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;
}
