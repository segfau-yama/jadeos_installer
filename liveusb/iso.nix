{
  config,
  lib,
  pkgs,
  modulesPath,
  jadeInstaller,
  ...
}:
let
  installerUser = "installer";
  installerTemplates = pkgs.runCommandLocal "jade-installer-templates" { } ''
    mkdir -p "$out"
    cp -r ${./templates}/. "$out"/
  '';
  launchInstaller = pkgs.writeShellScript "launch-jade-installer" ''
    export GDK_BACKEND=wayland,x11
    export WEBKIT_DISABLE_DMABUF_RENDERER=1
    export WLR_RENDERER_ALLOW_SOFTWARE=1
    export XDG_CURRENT_DESKTOP=jade-installer
    export XDG_SESSION_DESKTOP=jade-installer
    export XDG_SESSION_TYPE=wayland

    exec ${pkgs.dbus}/bin/dbus-run-session \
      ${lib.getExe pkgs.cage} -s -- \
      ${lib.getExe jadeInstaller}
  '';
in
{
  imports = [
    "${modulesPath}/installer/cd-dvd/installation-cd-minimal.nix"
  ];

  nix.settings.experimental-features = [
    "nix-command"
    "flakes"
  ];

  networking.hostName = "jade-installer";
  networking.networkmanager.enable = true;
  networking.wireless.enable = lib.mkForce false;

  time.timeZone = "Asia/Tokyo";
  i18n.defaultLocale = "ja_JP.UTF-8";
  console.keyMap = "jp106";

  boot.supportedFilesystems = [
    "ext4"
    "vfat"
  ];

  hardware.graphics.enable = true;
  fonts.fontconfig.enable = true;
  fonts.packages = with pkgs; [
    noto-fonts
    noto-fonts-cjk-sans
    noto-fonts-emoji
  ];

  services.dbus.enable = true;
  services.greetd = {
    enable = true;
    restart = false;
    settings = {
      initial_session = {
        user = installerUser;
        command = launchInstaller;
      };
      default_session = {
        user = "greeter";
        command = "${pkgs.greetd.greetd}/bin/agreety --cmd ${launchInstaller}";
      };
    };
  };

  security.sudo = {
    enable = true;
    wheelNeedsPassword = false;
    extraRules = [
      {
        users = [ installerUser ];
        commands = [
          {
            command = "ALL";
            options = [
              "NOPASSWD"
              "SETENV"
            ];
          }
        ];
      }
    ];
  };

  users.users.${installerUser} = {
    isNormalUser = true;
    extraGroups = [
      "networkmanager"
      "wheel"
    ];
    initialPassword = "";
  };

  environment.etc."jadeos-installer/templates".source = installerTemplates;

  environment.systemPackages = with pkgs; [
    cage
    dosfstools
    e2fsprogs
    git
    jadeInstaller
    parted
    util-linux
  ];

  environment.variables = {
    GDK_BACKEND = "wayland,x11";
    WEBKIT_DISABLE_DMABUF_RENDERER = "1";
    WLR_RENDERER_ALLOW_SOFTWARE = "1";
  };

  image.baseName = lib.mkForce "jadeos-installer";

  isoImage = {
    volumeID = "JADEOS_INSTALLER";
  };

  system.nixos.tags = [
    "jade-installer"
    "gui-liveusb"
  ];
  system.stateVersion = "25.05";
}
