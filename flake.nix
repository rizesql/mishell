{
  description = "shell - <description>";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, advisory-db }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        crane' = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = crane'.cleanCargoSource ./.;
        args = {
          inherit src;
          buildInputs = with pkgs; [ openssl ];
          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
          ] ++ lib.optionals stdenv.isDarwin [
            libiconv
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];
        };

        artifacts = crane'.buildDepsOnly (args // {
          doCheck = false;
        });
        bin = crane'.buildPackage (args // {
          inherit artifacts;
        });
      in
      {
        checks = {
          inherit bin;

          clippy = crane'.cargoClippy (args // {
            cargoArtifacts = artifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });
          doc = crane'.cargoDoc (args // {
            cargoArtifacts = artifacts;
          });
          fmt = crane'.cargoFmt { inherit src; };
          audit = crane'.cargoAudit { inherit src advisory-db; };
          test = crane'.cargoNextest (args // {
            cargoArtifacts = artifacts;
            partitions = 1;
            partitionType = "count";
          });
        };

        packages = {
          inherit bin;
          default = bin;
        };
        devShells.default = crane'.devShell {
          checks = self.checks.${system};
          inputsFrom = [ bin ];
          packages = with pkgs; [
            nixd
            cargo-audit
            cargo-deny
            cargo-edit
            cargo-generate
            cargo-nextest
            cargo-watch
            rust-analyzer
            rustup
            bunyan-rs
          ];
          env = {
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };
        };
      }
    );
}
