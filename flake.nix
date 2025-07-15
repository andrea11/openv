{
  description = "Rust Project Template";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs =
    {
      flake-utils,
      git-hooks,
      nixpkgs,
      self,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        mainPkg = pkgs.rustPlatform.buildRustPackage {
          pname = "openv";
          version = "0.0.1";

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      in
      rec {
        checks = {
          pre-commit-check = git-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              actionlint.enable = true;
              check-yaml.enable = true;
              clippy = {
                enable = true;
                settings.allFeatures = true;
              };
              commitizen.enable = true;
              nixfmt-rfc-style.enable = true;
              rustfmt.enable = true;
              shellcheck.enable = true;
            };
          };
        };

        packages = {
          default = mainPkg;
        };

        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              cargo
              clippy
              rust-analyzer
              rustfmt
            ];

            buildInputs = self.checks.${system}.pre-commit-check.enabledPackages;

            shellHook = ''
              ${self.checks.${system}.pre-commit-check.shellHook}

              echo 🔨 Rust DevShell
            '';
          };

        };

        hydraJobs = packages;
      }
    );
}
