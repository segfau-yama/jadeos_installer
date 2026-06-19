{ inputs, ... }:
{
  flake.nixosConfigurations = inputs.self.lib.mkNixos __JADE_SYSTEM_ARCH__ __JADE_HOSTNAME__;
}
