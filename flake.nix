{
  description = "CLI to print using GTK";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          # Explicitely include CUPS support on macOS as this package would be useless on there otherwise
          gtk4 =
            if pkgs.stdenv.hostPlatform.isDarwin then pkgs.gtk4.override { cupsSupport = true; } else pkgs.gtk4;
        in
        {
          gtk-print-rs = pkgs.callPackage ./. { inherit gtk4; };
          default = self.packages.${system}.gtk-print-rs;
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.gtk-print-rs ];
            packages = with pkgs; [
              rustc
              cargo
              rust-analyzer
              rustfmt
              clippy
            ];
          };
        }
      );
    };
}
