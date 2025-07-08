{
  description = "Helps you to identify outdated helm charts in your argocd instance.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustWithExtensions = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "cargo"
            "rust-src"
            "rust-std"
            "clippy"
          ];
        };

        craneLib = (crane.mkLib pkgs);

        vaultpipe = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          doNotSign = true;

          buildInputs =
            [
              # Add additional build inputs here
              pkgs.pkg-config
              pkgs.libiconv
              pkgs.openssl
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libpng
              pkgs.zlib
            ];
        };
      in
      {
        checks = {
          inherit vaultpipe;
        };

        packages.default = vaultpipe;

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          packages = with pkgs; [
            rustWithExtensions
            cargo-audit
            cargo-edit
            cargo-watch
            libiconv
            rust-analyzer
          ];
        };
      }
    );
}
