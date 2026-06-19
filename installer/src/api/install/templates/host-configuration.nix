{ inputs, lib, pkgs, ... }:
{
  flake.modules.nixos.__JADE_HOSTNAME_ATTR__ = {
    lib,
    ...
  }: {
    imports =
      (with inputs.self.modules.nixos; [
        system-base
        home-manager
        locale
        fcitx5
        audio
        desktop
      ])
      ++ [
        inputs.self.modules.nixos.__JADE_USERNAME_ATTR__
        __JADE_HARDWARE_PATH__
      ];

    networking.hostName = __JADE_HOSTNAME__;

    my.hardware.gpu = lib.mkDefault __JADE_GPU__;
    my.hardware.cpu = lib.mkDefault __JADE_CPU__;

    console.keyMap = "jp106";
    i18n.defaultLocale = "ja_JP.UTF-8";
    time.timeZone = "Asia/Tokyo";
  };
}
