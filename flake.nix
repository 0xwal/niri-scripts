# vim: set ts=2 sw=2 et:
{
  description = "Niri scripts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    nixpkgs-unstable = {
      # NOTE: WE NEED CARGO 1.88+ AS IT HAS rust#132833
      # NOW STABLE NIX HAS ONLY CARGO 1.86
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      nixpkgs-unstable,
      ...
    }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      unstablePkgs = nixpkgs-unstable.legacyPackages.${system};

      scriptisto = pkgs.scriptisto;

      # helper to wrap an rs script
      wrap =
        scriptFile: name: isRust:
        if isRust then
          pkgs.stdenv.mkDerivation rec {
            pname = builtins.baseNameOf scriptFile;
            version = "0.1.0";

            src = scriptFile;
            dontUnpack = true;

            nativeBuildInputs = [ pkgs.makeWrapper ];
            buildInputs = [ pkgs.scriptisto ];

            installPhase = ''
              mkdir -p $out/bin
              cp $src $out/${pname}.rs
              makeWrapper ${pkgs.scriptisto}/bin/scriptisto $out/bin/${name} \
                --add-flags "$out/${pname}.rs" \
                --prefix PATH : ${
                  pkgs.lib.makeBinPath [
                    unstablePkgs.rustc
                    unstablePkgs.cargo
                    unstablePkgs.rustPackages.clippy
                    pkgs.gcc
                    pkgs.pkg-config
                  ]
                }
            '';

            meta = with pkgs.lib; {
              description = "Niri scripts";
              homepage = "https://github.com/0xWal/niri-scripts";
              license = licenses.mit;
              platforms = platforms.linux;
              mainProgram = name;
            };
          }
        else
          # For non-Rust scripts: Just symlink it or wrap in a trivial script
          pkgs.writeShellScriptBin name ''
            exec ${scriptFile} "$@"
          '';

      wallpaper = wrap ./wallpaper-per-workspace "niri-wallpaper-per-workspace" true;
      sticky = rec {
        daemon = (wrap ./support-sticky-floating "niri-sticky-daemon") true;
        client =
          (wrap (pkgs.writeShellScript "niri-sticky-client-wrapper" ''
              ${pkgs.lib.getExe daemon} toggle-sticky
          '') "niri-sticky-client")
            false;
      };
      screenshot = wrap ./screenshot "niri-screenshot" true;

    in
    {
      packages.${system} = {
        sticky = sticky;
        wallpaper = wallpaper;
        screenshot = screenshot;
      };

      nixosConfigurations.default = nixpkgs.lib.nixosSystem {
        system = system;
        modules = [
          (import "${nixpkgs}/nixos/modules/installer/cd-dvd/installation-cd-base.nix")
          {
            imports = [
              self.nixosModules.default

            ];

            users.users.niri = {
              isNormalUser = true;
              description = "Niri Test User";
              extraGroups = [ "wheel" ]; # for sudo
            };

            services.getty.autologinUser = pkgs.lib.mkForce "niri";
            environment.systemPackages = [
              sticky.daemon
              sticky.client
              wallpaper
              screenshot
            ];
          }
          {
            hardware.graphics.enable = true;
            programs.niri.enable = true;
            niri-scripts = {
              enable = true;
              screenshot = {
                enable = true;
                dir = "~/Pictures/Screenshots";
              };
              wallpaper-per-workspace = {
                enable = true;
                dir = "~/.wallpapers";
              };
              sticky-window.enable = true;
            };
          }
        ];
      };

      devShells.${system}.default = pkgs.mkShell {
        packages = [
          sticky.daemon
          sticky.client
          wallpaper
          screenshot
        ];
      };

      # nixosModules.default = (
      #   import ./nix/niri-scripts.nix (with pkgs; {
      #     inherit
      #       config
      #       lib
      #       pkgs
      #       ;
      #     niriScriptsPkg = {
      #       inherit supportSticky wallpaper screenshot;
      #     };
      #   })
      # );

      nixosModules.default = import ./nix/niri-scripts.nix { inherit self; };
    };
}
