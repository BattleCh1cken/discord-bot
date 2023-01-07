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
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };

      craneLib = crane.lib.${system};
      fred = craneLib.buildPackage
        {
          src = craneLib.cleanCargoSource ./.;
        };

    in
    {
      checks = {
        inherit fred;
      };

      packages = {
        default = fred;
      };

      apps.${system}.default = flake-utils.lib.mkApp {
        drv = fred;
      };
      nixosModule =
        { config, options, lib, pkgs, ... }:
        let
          cfg = config.services.fred;
        in
        with lib; {
          options = {
            services.fred = {

              enable = mkOption {
                default = false;
                type = with types; bool;
                description = "Start the fred service for a user";
              };

              envFilePath = mkOption {
                type = with types; uniq str;
                description = "Path the .env file";
              };

              dbFilePath = mkOption {
                type = with types; uniq str;
                description = "Path the .env file";
              };

            };


          };
          config = mkIf cfg.enable
            {
              environment.systemPackages = [ fred ];
              systemd.services.fred = {
                wantedBy = [ "multi-user.target" ];
                after = [ "network.target" ];
                environment = {
                  DATABASE_URL = "sqlite://${cfg.dbFilePath}";
                };
                serviceConfig = {
                  Type = "simple";
                  EnvironmentFile = "${cfg.envFilePath}";
                  ExecStart = ''
                    ${fred}/bin/fred
                  '';
                };
              };
            };
        };

      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;
        shellHook = ''
          export $(cat .env)
        '';

        nativeBuildInputs = with pkgs; [
          sqlx-cli
          cargo
          rustc
          rustfmt
          rust-analyzer
        ];
      };
    };
}
