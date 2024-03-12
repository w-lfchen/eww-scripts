{
  description = "Some eww scripts";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustfmt
          rustc
          cargo
          sccache
          rust-analyzer
          clippy
        ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        shellHook = ''
          export RUSTC_WRAPPER=sccache
        '';
      };
      packages.${system} =
        let
          targets = [
            "eww-launch"
            "hyprland-current-window-title"
            "hyprland-workspaces"
          ];
          packages = nixpkgs.lib.genAttrs targets (
            name:
            pkgs.rustPlatform.buildRustPackage {
              inherit name;
              src = ./.;
              cargoBuildFlags = "--bin ${name}";
              cargoLock.lockFile = ./Cargo.lock;
            }
          );
        in
        packages
        // rec {
          all = pkgs.symlinkJoin {
            name = "eww-scripts";
            paths = builtins.attrValues packages;
          };
          default = all;
        };
      formatter.${system} = pkgs.nixfmt-rfc-style;
    };
}
