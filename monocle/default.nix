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

  cargoSha256 = "03214jv5grzbva034fw3wyw7m599cgl3ass7kn8fh3c8l75jwf16";

  #nativeBuildInputs = [pkgs.pkgconfig];
  buildInputs = [
    pkgs.libdrm
  ]; #with pkgs; [openblas mkl openssl];

  meta = with lib; {
    description = "Software for a single pixel camera";
    homepage = "https://github.com/taktoa/monocle";
    license = with licenses; [ asl20 ];
    maintainers = with maintainers; [ taktoa ];
    mainProgram = "monocle";
  };
}
