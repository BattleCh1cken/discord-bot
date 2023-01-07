{ config, pkgs, lib, fred, ... }:
let
  cfg = config.services.fred;
in
with lib; {
  options = {
    services.fred = {
      enable = mkOption {
        default = false;
        type = with types; bool;
        description = "Start the fred server for a user";
      };


      #envFile = mkOption {
        #type = with types; uniq str;
        #description = "Path to .env file";
      #};
      config = mkIf cfg.enable {


        systemd.services.fred = {
          wantedBy = [ "multi-user.target" ];
          after = [ "network.target" ];
          description = "Start the fred server";

          serviceConfig = {
            #environmentFile = "${cfg.envFile}";
            Type = "simple";
            ExecStart = "${fred}/bin/fred";
          };
        };

        environment.systemPackages = [ fred ];
      };
    };
  };
}
