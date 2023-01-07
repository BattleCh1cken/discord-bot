{ config, optsions, lib, pkgs, fred, ... }:
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
      envFile = mkOption {
        type = with types; uniq str;
        description = "Path to .env file";
      };
    };
  };

  config = mkIf cfg.enable {
    environment.systemPackages = [ fred ];
    systemd.services.fred = {
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];
      serviceConfig = {
        environmentFile = "${cfg.envFile}";
        Type = "simple";
        ExecStart = ''
          ${fred}/bin/fred
        '';
      };
    };

  };
}
