{
  description = "JadeOS Live USB image with the GUI installer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
      jadeInstaller = pkgs.callPackage ./installer-package.nix { };
    in
    {
      packages.${system} = {
        jade-installer = jadeInstaller;
        iso = self.nixosConfigurations.jade-liveusb.config.system.build.isoImage;
        default = self.packages.${system}.iso;
      };

      nixosConfigurations.jade-liveusb = nixpkgs.lib.nixosSystem {
        inherit system;
        specialArgs = {
          inherit jadeInstaller;
        };
        modules = [
          ./iso.nix
        ];
      };
    };
}
