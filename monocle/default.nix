{ lib, stdenv
, fetchFromGitHub
, rustPlatform
, pkgs
}:

rustPlatform.buildRustPackage rec {
  pname = "monocle";
  version = "0.1.0";

  src = lib.cleanSourceWith {
    filter = p: t: !(t == "directory" && baseNameOf p == "target");
    src = lib.cleanSource ./.;
  };

  cargoSha256 = "10rvxjdl3w5hjrxjz7qmrk1vrf1ybis797ydp5pbpxymjqkcndzm";

  #nativeBuildInputs = [pkgs.pkgconfig];
  buildInputs = [
    pkgs.libdrm
    pkgs.openblas
  ]; #with pkgs; [openblas mkl openssl];

  meta = with lib; {
    description = "Software for a single pixel camera";
    homepage = "https://github.com/taktoa/monocle";
    license = with licenses; [ asl20 ];
    maintainers = with maintainers; [ taktoa ];
    mainProgram = "monocle";
  };
}
