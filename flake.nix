{
  description = "Writing a C Compiler";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    nix-core = {
      url = "github:Cloud-Scythe-Labs/nix-core";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.fenix.follows = "fenix";
    };
  };

  outputs =
    { self
    , nixpkgs
    , crane
    , fenix
    , flake-utils
    , advisory-db
    , nix-core
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      inherit (pkgs) lib;
      pkgs = nixpkgs.legacyPackages.${system};

      rustToolchain = nix-core.toolchains.${system}.mkRustToolchainFromTOML
        ./.rust-toolchain.toml
        "sha256-s1RPtyvDGJaX/BisLT+ifVfuhDT1nZkZ1NcK8sbwELM=";
      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain.fenix-pkgs;

      src = craneLib.cleanCargoSource ./.;

      commonArgs = {
        inherit src;
        strictDeps = true;

        buildInputs = [ ] ++ lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];
      };

      craneLibLLvmTools = craneLib.overrideToolchain
        (fenix.packages.${system}.complete.withComponents [
          "cargo"
          "llvm-tools"
          "rustc"
        ]);

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      my-crate = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
      });
    in
    {
      checks = {
        inherit my-crate;

        my-crate-clippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });

        my-crate-doc = craneLib.cargoDoc (commonArgs // {
          inherit cargoArtifacts;
        });

        my-crate-fmt = craneLib.cargoFmt {
          inherit src;
        };

        my-crate-toml-fmt = craneLib.taploFmt {
          src = lib.sources.sourceFilesBySuffices src [ ".toml" ];
        };

        my-crate-audit = craneLib.cargoAudit {
          inherit src advisory-db;
        };

        my-crate-deny = craneLib.cargoDeny {
          inherit src;
        };

        my-crate-nextest = craneLib.cargoNextest (commonArgs // {
          inherit cargoArtifacts;
          partitions = 1;
          partitionType = "count";
        });
      };

      packages = {
        default = my-crate;
      } // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
        my-crate-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // {
          inherit cargoArtifacts;
        });
      };

      apps.default = flake-utils.lib.mkApp {
        drv = my-crate;
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};

        packages = with pkgs; [
          gcc
          gdb
          python39
        ];
      };

      formatter = pkgs.nixpkgs-fmt;
    });
}
