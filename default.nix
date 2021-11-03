let
  sources = import nix/sources.nix;
  common = import ./nix/common.nix;
  pkgs = import sources.nixpkgs {
    config = {};
    overlays = [
      # (import sources.nixpkgs-mozilla)
      (import sources.rust-overlay)
      (import "${sources.cargo2nix}/overlay")
      #(import (builtins.fetchTarball {
      #  url = "https://github.com/cargo2nix/cargo2nix/archive/master.tar.gz";
      #}))
      # (self: super: {
      #   rustc = (self.buildPackages.rustChannelOf {
      #     date = "2021-11-15";
      #     channel = "nightly";
      #   });
      #   cargo = self.rustc;
      #   #inherit (self.rust.packages.nightly) cargo rustc;
      #   #inherit (self.rustPackages) rustPlatform;
      #   #rustPlatform = self.makeRustPlatform { inherit (self) cargo rustc; };
      # })
      (self: super: {
        cargo2nix = ((self.rustBuilder.makePackageSet' {
          packageFun = import "${sources.cargo2nix}/Cargo.nix";
          rustChannel = "1.56.1";
          packageOverrides = p: p.rustBuilder.overrides.all;
        }).workspace.cargo2nix {}).bin;
        monoclePkgs = (self.rustBuilder.makePackageSet' {
          packageFun = import ./monocle/Cargo.nix;
          workspaceSrc = self.lib.cleanSourceWith {
            filter = p: t: !(t == "directory" && baseNameOf p == "target");
            src = self.lib.cleanSource ./monocle;
          };
          rustChannel = "nightly";
          packageOverrides = pkgs: pkgs.rustBuilder.overrides.all;
        });
        monocle = (self.monoclePkgs.workspace.monocle {}).bin
          .overrideAttrs (old: {
            RUSTFLAGS = (old.RUSTFLAGS or "") + " -C target-cpu=cortex-a72";
          });
      })
      common
    ];
  };
  # nix build -L -f ~/apps/rpi/rpi-tools/ pkgsCross.armv7l-hf-multiplatform.pkgsStatic.msd.rootDir
  # nix build -L -f ~/apps/rpi/rpi-tools/ pkgsCross.armv7l-hf-multiplatform.android-auto.rootDir
in pkgs
