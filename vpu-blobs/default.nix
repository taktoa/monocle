let
  pkgs = import (builtins.fetchTarball {
    url = "https://github.com/input-output-hk/nixpkgs/archive/0ee0489d42e.tar.gz";
    sha256 = "1ldlg2nm8fcxszc29rngw2893z8ci4bpa3m0i6kfwjadfrcrfa42";
  }) { system = "x86_64-linux"; };
in
  pkgs.pkgsCross.vc4.stdenv.mkDerivation {
    name = "vpu-blobs";
    src = ../vpu-blobs;
  }
