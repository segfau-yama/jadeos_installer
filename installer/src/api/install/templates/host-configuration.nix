{ inputs, lib, pkgs, ... }:
{
  imports = with inputs.self.modules.nixos; [
    base
    home-manager
    hyprland
  ] ++ [ ./hardware-configuration.nix ];

  networking.hostName = __JADE_HOSTNAME__;

  my.hardware.gpu = lib.mkDefault __JADE_GPU__;
  my.hardware.cpu = lib.mkDefault __JADE_CPU__;
  my.installDisk = {
    boot = "/dev/disk/by-label/boot";
    root = "/dev/disk/by-label/nixos";
    swap = "";
  };

  console.keyMap = "jp106";
  i18n.defaultLocale = "ja_JP.UTF-8";
  time.timeZone = "Asia/Tokyo";

  users.users.__JADE_USERNAME_ATTR__ = {
    isNormalUser = true;
    description = __JADE_USERNAME__;
    extraGroups = [
      "wheel"
      "networkmanager"
      "audio"
      "video"
      "input"
      "seat"
    ];
    shell = pkgs.zsh;
  };

  programs.zsh.enable = true;

  my.desktop.hyprlandUsers = [ __JADE_USERNAME__ ];

  home-manager.users.__JADE_USERNAME_ATTR__ = {
    imports = with inputs.self.modules.homeManager; [
      base
      hyprland
    ];

    my.capabilities = {
      user_interface = "gui";
      window_manager = "hyprland";
    };

    home.username = __JADE_USERNAME__;
    home.homeDirectory = __JADE_HOME_DIRECTORY__;
    home.stateVersion = "25.05";

    programs.home-manager.enable = true;
  };
}
