{
  description = "The source behind msfjarvis.dev";

  inputs = {
    devshell.url = "github:numtide/devshell/master";
    flake-utils.url = "github:numtide/flake-utils/master";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    nil.url = "github:oxalica/nil/main";
    devshell.inputs.flake-utils.follows = "flake-utils";
    devshell.inputs.nixpkgs.follows = "nixpkgs";
    nil.inputs.flake-utils.follows = "flake-utils";
    nil.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, devshell, flake-utils, nil, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlay ];
          config = { allowUnfree = true; };
        };
        nil_latest = nil.outputs.packages.${system}.nil;
      in {
        devShell = pkgs.devshell.mkShell {
          name = "build-report-ingestor-shell";
          bash = { interactive = ""; };
          packages = with pkgs; [
            deno
            git
            nil_latest
            nixfmt
            (vscode-with-extensions.override {
              vscodeExtensions = with vscode-extensions;
                [ ] ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [
                  {
                    name = "daybreak-theme";
                    publisher = "mtdmali";
                    version = "1.2.0";
                    sha256 =
                      "1zyl9kri2pna9jsvcjvf0vqz6xiq3v3j4p64glvnn0cgbaqsmmy3";
                  }
                  {
                    name = "intellij-idea-keybindings";
                    publisher = "k--kato";
                    version = "1.5.4";
                    sha256 =
                      "1y759wa4rz2n5a1cjpbj7q0n52932pv30ymhvisq9zva1cwp04yx";
                  }
                  {
                    name = "indent-rainbow";
                    publisher = "oderwat";
                    version = "8.3.1";
                    sha256 =
                      "0iwd6y2x2nx52hd3bsav3rrhr7dnl4n79ln09picmnh1mp4rrs3l";
                  }
                  {
                    name = "nix-ide";
                    publisher = "jnoortheen";
                    version = "0.2.1";
                    sha256 =
                      "0bibb3r4cb7chnx6vpyl41ig12pc0cbg0sb8f2khs52c71nk4bn8";
                  }
                  {
                    name = "vscode-deno";
                    publisher = "denoland";
                    version = "3.14.1";
                    sha256 =
                      "01s12mnchnn8786m5b58g7pmgbm805s12vmgyhknkx9zlgajvyln";
                  }
                ];
            })
          ];
          commands = [
            {
              name = "fmt";
              category = "development";
              command = "nixfmt flake.nix && deno fmt";
              help = "Run formatting jobs";
            }
            {
              name = "lint";
              category = "development";
              command = "deno lint";
              help = "Run linter";
            }
            {
              name = "run";
              category = "development";
              command = "deno run main.ts";
              help = "Run the main server";
            }
            {
              name = "t";
              category = "development";
              command = "deno test";
              help = "Run tests";
            }
          ];
        };
      });
}
