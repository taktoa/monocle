let
  sources = import nix/sources.nix;
  common = import ./nix/common.nix;
  telescope = import ./nix/telescope.nix;
  pkgs = import sources.nixpkgs {
    overlays = [ common telescope ];
  };
  # nix build -L -f ~/apps/rpi/rpi-tools/ pkgsCross.armv7l-hf-multiplatform.pkgsStatic.msd.rootDir
  # nix build -L -f ~/apps/rpi/rpi-tools/ pkgsCross.armv7l-hf-multiplatform.android-auto.rootDir
in pkgs
