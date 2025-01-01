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

      individualCrateArgs = commonArgs // {
        inherit cargoArtifacts;
        inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
        doCheck = false;
      };

      fileSetForCrate = crate: deps: lib.fileset.toSource {
        root = ./.;
        fileset = lib.fileset.unions ([
          ./Cargo.toml
          ./Cargo.lock
          (craneLib.fileset.commonCargoSources crate)
        ] ++ deps);
      };

      wacc = craneLib.buildPackage (individualCrateArgs // {
        pname = "wacc";
        cargoExtraArgs = "-p wacc-driver";
        src = fileSetForCrate ./crates/driver [ ];
      });
      wacc-lexer = craneLib.buildPackage (individualCrateArgs // {
        pname = "wacc-lexer";
        cargoExtraArgs = "-p wacc-lexer";
        src = fileSetForCrate ./crates/lexer [
          ./crates/derive-token
          ./crates/tokengen
        ];
      });
    in
    {
      checks = {
        inherit wacc wacc-lexer;

        clippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });

        fmt = craneLib.cargoFmt {
          inherit src;
        };

        toml-fmt = craneLib.taploFmt {
          src = lib.sources.sourceFilesBySuffices src [ ".toml" ];
        };

        nextest = craneLib.cargoNextest (commonArgs // {
          inherit cargoArtifacts;
          partitions = 1;
          partitionType = "count";
        });

        # TODO: Uncomment if compile times get too long
        # hakari = craneLib.mkCargoDerivation {
        #   inherit src;
        #   pname = "hakari";
        #   cargoArtifacts = null;
        #   doInstallCargoArtifacts = false;

        #   buildPhaseCargoCommand = ''
        #     cargo hakari generate --diff  # workspace-hack Cargo.toml is up-to-date
        #     cargo hakari manage-deps --dry-run  # all workspace crates depend on workspace-hack
        #     cargo hakari verify
        #   '';

        #   nativeBuildInputs = [
        #     pkgs.cargo-hakari
        #   ];
        # };
      };

      packages = {
        inherit wacc;
        default = wacc;
      } // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
        my-crate-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // {
          inherit cargoArtifacts;
        });
      };

      apps = {
        wacc = flake-utils.lib.mkApp {
          drv = wacc;
        };
        default = flake-utils.lib.mkApp {
          drv = wacc;
        };
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};

        packages = with pkgs; [
          nil
          nixpkgs-fmt

          gcc
          gdb
          python39
          mdbook
        ];
      };

      formatter = pkgs.nixpkgs-fmt;
    });
}
