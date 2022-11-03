{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = {nixpkgs, flake-utils, cargo2nix, ...}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlays.default];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.61.0";
          packageFun = import ./Cargo.nix;
        };

      in rec {
        packages = rec {
          # nix build .#packages.x86_64-linux.goldvalley
          # nix build .#goldvalley
          goldvalley = (rustPkgs.workspace.goldvalley {}).bin;
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