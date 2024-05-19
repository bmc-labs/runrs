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

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        # alternatively, to use a rust-toolchain.toml - which we'll want once runrs becomes stable:
        # rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = crane.lib.${system}.overrideToolchain rustToolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
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
          doCheck = false;
        });
      in
      {
        packages = {
          default = runrs;
          inherit runrs;
        };
      }
    );
}
