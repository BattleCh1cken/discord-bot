{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    let

      system = "x86_64-linux";
      pkgs = (import nixpkgs) {
        inherit system;
      };


      naersk' = pkgs.callPackage naersk { };
      migrations = ./migrations;
      sqlx-db = pkgs.runCommand "sqlx-db-prepare"
        {
          nativeBuildInputs = [ pkgs.sqlx-cli ];
        } ''
        mkdir $out
        export DATABASE_URL=sqlite:$out/db.sqlite3
        sqlx database create
        sqlx migrate --source ${migrations} run
      '';

    in
    rec {
      fred = naersk'.buildPackage {
        src = ./.;

        # Haha offline mode more like, my sanity is gone
        overrideMain = old: {
          linkDb = ''
            export DATABASE_URL=sqlite:${sqlx-db}/db.sqlite3
          '';

          preBuildPhases = [ "linkDb" ] ++ (old.preBuildPhases or [ ]);
        };
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

      # For `nix develop` (optional, can be skipped):

      devShells.${system}.default = pkgs.mkShell {
        devShell = ''
        export $(cat .env)
        '';
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rust-analyzer
          rustfmt
          sqlx-cli
          sqliteman
          sqlite
        ];
      };
    };
}
