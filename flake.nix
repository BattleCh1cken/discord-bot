{
  # def not stolen from robbb
  description = "discord bot for the Area 53 discord server";
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, crane, rust-overlay, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        # Common Args
        commonArgs = {
          pname = "edward";
          version = "0.0.1";
          src = builtins.path { path = pkgs.lib.cleanSource ./.; name = "edward"; };
          nativeBuildInputs = with pkgs; [ rust-toolchain pkg-config ];
          buildInputs = with pkgs; [ openssl sqlx-cli rust-analyzer sqlite ];
        };
        # Use the toolchain from the `rust-toolchain` file
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
        craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;

        # Build Deps so We don't have to build them everytime
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Run Cargo Fmt
        edwardFmt = craneLib.cargoFmt (commonArgs // { inherit cargoArtifacts; });

        # Run Clippy (only if cargo fmt passes)
        edwardClippy = craneLib.cargoClippy (commonArgs // {
          cargoArtifacts = edwardFmt;
          cargoClippyExtraArgs = "-- -D warnings";
        });

        # Build Robb (only if all above tests pass)
        edward = craneLib.buildPackage
          (commonArgs // { cargoArtifacts = edwardClippy; });
      in
      {
        # `nix flake check` (build, fmt and clippy)
        checks = { inherit edward; };

        # `nix build`
        packages.default = edward;

        # `nix run`
        apps.default = utils.lib.mkApp { drv = edward; };

        # `nix develop`
        devShells.default = pkgs.mkShell {
          shellHook = ''
            export $(cat .env)
          '';
          inputsFrom = builtins.attrValues self.checks;

          # Extra inputs can be added here
          packages = commonArgs.nativeBuildInputs ++ commonArgs.buildInputs;
        };
      });
}
