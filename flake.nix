{
  description = "Flake for my eww scripts";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustfmt
          rustc
          cargo
          sccache
          rust-analyzer
        ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        shellHook = ''
          export RUSTC_WRAPPER=sccache
        '';
      };
      packages.${system} = {
        hyprland-workspaces = pkgs.rustPlatform.buildRustPackage {
          name = "hyprland-workspaces";
          src = ./hyprland-workspaces;
          cargoLock = {
            lockFile = ./hyprland-workspaces/Cargo.lock;
          };
          nativeBuildInputs = [];
          buildInputs = [];
        };
        hyprland-current-window-title = pkgs.rustPlatform.buildRustPackage {
          name = "hyprland-current-window-title";
          src = ./hyprland-current-window-title;
          cargoLock = {
            lockFile = ./hyprland-current-window-title/Cargo.lock;
          };
          nativeBuildInputs = [];
          buildInputs = [];
        };
        eww-launch = pkgs.rustPlatform.buildRustPackage {
          name = "eww-launch";
          src = ./eww-launch;
          cargoLock = {
            lockFile = ./eww-launch/Cargo.lock;
          };
          nativeBuildInputs = [];
          buildInputs = [];
        };
      };
    };
}
