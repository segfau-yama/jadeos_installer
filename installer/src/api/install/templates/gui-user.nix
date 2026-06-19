{ inputs, ... }:
let
  username = __JADE_USERNAME__;
in
{
  flake.modules.nixos."${username}" = { pkgs, ... }: {
    users.users."${username}" = {
      isNormalUser = true;
      description = username;
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

    home-manager.users."${username}" = {
      imports = [ inputs.self.modules.homeManager."${username}" ];
    };
  };

  flake.modules.homeManager."${username}" = { ... }: {
    imports = with inputs.self.modules.homeManager; [
      desktop
    ];

    home.username = username;
    home.homeDirectory = "/home/${username}";
    home.stateVersion = "25.05";

    programs.home-manager.enable = true;
  };
}
