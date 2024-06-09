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

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        inherit (pkgs) lib;

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        # alternatively, to use a rust-toolchain.toml - which we'll want once runrs becomes stable:
        # rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib nixpkgs.legacyPackages.${system}).overrideToolchain rustToolchain;

        # Required to NOT filter out SQL files from the source tree. We need them for the build
        # process because we compile them into the binary, allowing us to run migrations from just
        # the binary without additional tooling.
        sqlFilter = path: _type: null != builtins.match "^.*/migrations/.+\.sql$" path;
        sqlOrCargo = path: type: (sqlFilter path type) || (craneLib.filterCargoSources path type);

        src = lib.cleanSourceWith {
          src = craneLib.path ./.; # The original, unfiltered source
          filter = sqlOrCargo;
        };

        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs =
            with pkgs;
            [ openssl ] ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];

          nativeBuildInputs = with pkgs; [
            pkg-config
            # This switches rustfmt to the nightly channel.
            rust-bin.nightly.latest.rustfmt
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        runrs = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

        runrs-fmt = craneLib.cargoFmt (commonArgs // { inherit cargoArtifacts; });

        runrs-clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets --no-deps -- --deny warnings --deny clippy::all";
          }
        );

        runrs-nextest = craneLib.cargoNextest (commonArgs // { inherit cargoArtifacts; });

        runrs-docker-image = pkgs.dockerTools.buildLayeredImage {
          name = "ghcr.io/bmc-labs/runrs";
          tag =
            if (builtins.pathExists ./version) then
              lib.removeSuffix "\n" (builtins.readFile ./version)
            else
              "latest";
          #
          # "created" option takes an ISO timestamp or "now"; defaults to UNIX epoch.
          #
          # Using the default or a fixed timestamp (i.e., the time the git tag was created for a
          # release) makes the Docker image bit-level reproducible. Using "now" breaks this.
          #
          # created = "now";
          contents = with pkgs.dockerTools; [
            usrBinEnv
            binSh
            caCertificates
            runrs
          ];
          config = {
            Cmd = [ "${runrs}/bin/runrs" ];
            # These labels are required for GitHub to correctly associate the image with, and thus
            # inherit visibility from, the repository.
            Labels = {
              "org.opencontainers.image.source" = "https://github.com/bmc-labs/runrs";
              "org.opencontainers.image.description" = "Manage CI runners via a REST API.";
              "org.opencontainers.image.licenses" = "Apache-2.0";
            };
            # Not technically required to specify here, but a hint for runtime environments.
            ExposedPorts = {
              "3000/tcp" = { };
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
          inherit
            runrs
            runrs-fmt
            runrs-clippy
            runrs-nextest
            ;
        };

        devShells.default = craneLib.devShell {
          inputsFrom = [ runrs ];
          checks = self.checks.${system};
        };
      }
    );
}
