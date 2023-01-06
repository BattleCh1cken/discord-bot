{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        craneLib = crane.lib.${system};
        fred = craneLib.buildPackage
          {
            src = craneLib.cleanCargoSource ./.;

            buildInputs = [
              # Add additional build inputs here
            ];
          };

      in
      {
        checks = {
          inherit fred;
        };

        packages = {
          default = fred;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = fred;
        };

        nixosModule = import ./service.nix;

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;
          shellHook = ''
            export $(cat .env)
          '';

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            sqlx-cli
            cargo
            rustc
            rustfmt
            rust-analyzer
          ];
        };
      });
}
