{
  description = "dev shell";

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
        wasm-bindgen-overlay = (final: prev: {
          wasm-bindgen-cli = prev.wasm-bindgen-cli.override {
            version    = "0.2.99";
            hash       = "sha256-1AN2E9t/lZhbXdVznhTcniy+7ZzlaEp/gwLEAucs6EA=";
            cargoHash  = "sha256-DbwAh8RJtW38LJp+J9Ht8fAROK9OabaJ85D9C/Vkve4=";
          };
        });

        overlays = [ (import rust-overlay) wasm-bindgen-overlay ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.nightly."2024-10-20".default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
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
            # tools
            pkgs.just

            # game
            rust
            pkgs.openssl
            pkgs.wasm-bindgen-cli

            # web
            pkgs.pnpm_9
            pkgs.nodejs_23
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
