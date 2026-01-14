# vim: set ts=2 sw=2 et:
{ self }:
{
  config,
  lib,
  pkgs,
  ...
}:

with lib;
let
  cfg = config.niri-scripts;
  selfPkgs = self.packages.${pkgs.system};
in
{
  options.niri-scripts = {
    enable = mkEnableOption "Enable all Niri scripts";

    screenshot = {
      enable = mkEnableOption "Enable screenshot script";
      dir = mkOption {
        type = types.str;
        default = "~/Screenshots";
        description = "Directory to save screenshots to";
      };
    };

    sticky-window = {
      enable = mkEnableOption "Enable sticky window support";
    };

    auto-consume = {
      enable = mkEnableOption "Enable auto-consume new windows script";
    };

    wallpaper-per-workspace = {
      enable = mkEnableOption "Enable wallpaper-per-workspace script";
      dir = mkOption {
        type = types.str;
        default = "~/wallpapers";
        description = "Directory containing wallpapers per workspace";
      };
    };
  };

  config = mkIf cfg.enable (mkMerge [
    (mkIf cfg.screenshot.enable {
      assertions = [
        {
          assertion = config.programs.niri.enable or false;
          message = "niri-scripts requires programs.niri.enable = true";
        }
      ];

      environment.systemPackages = [
        (pkgs.writeShellScriptBin "niri-screenshot" ''
          ${lib.getExe selfPkgs.screenshot} ${cfg.screenshot.dir}
        '')

        (pkgs.writeShellScriptBin "niri-screenshot-annotate" ''
          ${lib.getExe selfPkgs.screenshot} ${cfg.screenshot.dir} --annotate
        '')

        pkgs.scriptisto
        pkgs.grim
        pkgs.satty
        pkgs.slurp
        pkgs.wl-clipboard
      ];
    })

    (mkIf cfg.sticky-window.enable {
      environment.systemPackages = with selfPkgs.sticky; [
        daemon
        client
      ];

      # systemd.user.services.support-sticky-floating = {
      #   enable = true;
      #   description = "Niri sticky window support";
      #   serviceConfig = {
      #     ExecStart = "${lib.getExe selfPkgs.supportSticky}";
      #     After = "niri.service";
      #     Requires = "niri.service";
      #   };
      #   wantedBy = [ "default.target" ];
      # };
    })

    (mkIf cfg.wallpaper-per-workspace.enable {
      environment.systemPackages = [
        selfPkgs.wallpaper
        pkgs.swww
      ];

      # systemd.user.services.wallpaper-per-workspace = {
      #   enable = true;
      #   description = "Niri wallpaper per workspace";
      #   serviceConfig = {
      #     ExecStart = "${lib.getExe selfPkgs.wallpaper} ${cfg.wallpaper-per-workspace.dir}";
      #     Restart = "always";
      #     After = "niri.service";
      #     Requires = "niri.service";
      #   };
      #   wantedBy = [ "default.target" ];
      # };
    })

    (mkIf cfg.auto-consume.enable {
      environment.systemPackages = [
        selfPkgs.autoConsume
      ];

      # systemd.user.services.auto-consume = {
      #   enable = true;
      #   description = "Niri auto-consume new windows";
      #   serviceConfig = {
      #     ExecStart = "${lib.getExe selfPkgs.autoConsume}";
      #     Restart = "always";
      #     After = "niri.service";
      #     Requires = "niri.service";
      #   };
      #   wantedBy = [ "default.target" ];
      # };
    })
  ]);
}
