{
  description = "A project comparing a couple of http API options";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devenv.url = "github:cachix/devenv";
    devenv-root = {
      url = "file+file:///dev/null";
      flake = false;
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [inputs.devenv.flakeModule];
      systems = inputs.nixpkgs.lib.systems.flakeExposed;
      perSystem = {config, pkgs, ...}: {
        devenv.shells.default = {
          devenv.root = let
              root = builtins.readFile inputs.devenv-root.outPath;
            in
              pkgs.lib.mkIf (root != "") root;

          languages = {
            rust = {
              enable = true;
              rustflags = "--cfg tokio_unstable";
            };

            python = {
              enable = true;
              package = pkgs.python313;
              uv.enable = true;
            };
          };

          services = {
            postgres = {
              enable = true;
              listen_addresses = "127.0.0.1";
              initialScript = ''
                CREATE ROLE postgres WITH LOGIN SUPERUSER PASSWORD 'postgres';
              '';
            };

            opensearch.enable = true;
          };

          packages = [
            # rust
            pkgs.bacon
            pkgs.sqlx-cli
            pkgs.tokio-console
          ];
        };
      };
    };
}
