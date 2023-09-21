{
  description = "A very basic flake";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
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
        nativeBuildInputs = [  ];
        buildInputs = [  ];
        };
      };
    };
}
