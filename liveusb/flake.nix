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
      installerRuntimeLibraries = with pkgs; [
        atk
        cairo
        gdk-pixbuf
        glib
        gtk3
        libayatana-appindicator
        libxkbcommon
        libsoup_3
        openssl
        pango
        webkitgtk_4_1
        xorg.libX11
        xorg.libXinerama
        xorg.libXtst
      ];
    in
    {
      packages.${system} = {
        jade-installer = jadeInstaller;
        iso = self.nixosConfigurations.jade-liveusb.config.system.build.isoImage;
        default = self.packages.${system}.iso;
      };

      devShells.${system} =
        let
          build = pkgs.mkShell {
            name = "jadeos-installer-build-shell";
            inputsFrom = [ jadeInstaller ];
            packages = with pkgs; [
              cargo
              clang
              git
              nodejs
              nixfmt-rfc-style
              pkg-config
              rust-analyzer
              rustc
              rustfmt
            ];

            shellHook = ''
              export JADEOS_INSTALLER_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
              export DIOXUS_ASSET_ROOT="$JADEOS_INSTALLER_ROOT/installer"
              export GDK_BACKEND=wayland,x11
              export WEBKIT_DISABLE_DMABUF_RENDERER=1
              export WLR_RENDERER_ALLOW_SOFTWARE=1
              export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath installerRuntimeLibraries}:''${LD_LIBRARY_PATH:-}"

              echo "Entered jadeos-installer build shell."
              echo "  installer build: cd installer && cargo build --release"
              echo "  iso build:       nix build ./liveusb#iso"
            '';
          };
        in
        {
          inherit build;
          default = build;
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
