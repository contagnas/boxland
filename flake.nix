{
  description = "Boardmage dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem(system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.nightly."2024-10-20".default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        runtimeInputs = with pkgs; [
            alsa-lib 
            udev
            vulkan-loader
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
        ];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = runtimeInputs ++ [
            rust
            pkgs.openssl
          ];

          nativeBuildInputs = [
            pkgs.pkg-config
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeInputs}"
          '';
        };
      });
}
