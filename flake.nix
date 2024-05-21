{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, crane, flake-utils, ... }:
  flake-utils.lib.eachDefaultSystem
    (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        inherit (pkgs) lib;

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        # alternatively, to use a rust-toolchain.toml - which we'll want once runrs becomes stable:
        # rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = crane.lib.${system}.overrideToolchain rustToolchain;


        sqlFilter = path: _type: null != builtins.match "^.*/migrations/.+\.sql$" path;
        sqlOrCargo = path: type: (sqlFilter path type) || (craneLib.filterCargoSources path type);

        src = lib.cleanSourceWith {
          src = craneLib.path ./.; # The original, unfiltered source
          filter = sqlOrCargo;
        };

        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = with pkgs; [
            openssl
          ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.Security
          ];

          nativeBuildInputs = with pkgs; [ pkg-config ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        runrs = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

        runrs-fmt = craneLib.cargoFmt (commonArgs // {
          inherit cargoArtifacts;
          rustFmtExtraArgs = "--check --all";
        });

        runrs-clippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets --no-deps -- --deny warnings --deny clippy::all";
        });

        runrs-nextest = craneLib.cargoNextest (commonArgs // {
          inherit cargoArtifacts;
        });

        runrs-docker-image = pkgs.dockerTools.buildLayeredImage {
          name = "ghcr.io/bmc-labs/runrs";
          tag = "latest";
          contents = with pkgs.dockerTools; [
            usrBinEnv
            binSh
            caCertificates
            runrs
          ];
          config = {
            Cmd = [ "${runrs}/bin/runrs" ];
            Labels = {
              "org.opencontainers.image.source" = "https://github.com/bmc-labs/runrs";
              "org.opencontainers.image.description" = "Manage CI runners via a REST API.";
              "org.opencontainers.image.licenses" = "MIT";
            };
            ExposedPorts = {
              "3000/tcp" = {};
            };
          };
        };
      in
      {
        packages = {
          default = runrs;
          inherit runrs runrs-docker-image;
        };

        checks = {
          inherit runrs runrs-fmt runrs-clippy runrs-nextest;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ runrs ];
        };
      }
    );
}
