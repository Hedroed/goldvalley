{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {nixpkgs, flake-utils, rust-overlay, ...}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };
        myRustBuild = rustPlatform.buildRustPackage {
          pname = "goldvalley";
          version = "1.0.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

      in rec {
        packages = rec {
          # nix build .#packages.x86_64-linux.goldvalley
          # nix build .#goldvalley
          goldvalley = myRustBuild;
          # goldvalley = (rustPkgs.workspace.goldvalley {}).bin;
          # nix build
          default = goldvalley;
        };
        apps = rec {
          # nix run .#goldvalley
          goldvalley = { type = "app"; program = "${packages.default}/bin/goldvalley"; };
          # nix run
          # nix run github:hedroed/goldvalley
          default = goldvalley;
        };
      }
    );
}