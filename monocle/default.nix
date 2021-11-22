{ lib, stdenv
, fetchFromGitHub
, rustPlatform
, pkgs
}:

rustPlatform.buildRustPackage rec {
  pname = "monocle";
  version = "0.1.0";

  src = lib.cleanSource ./.;

  cargoSha256 = "sha256-mquZitH8iI+XnbRnPWYIaGu5WHqXBD5APCDHLjdCiRE=";

  nativeBuildInputs = [pkgs.pkgconfig];
  buildInputs = with pkgs; [openblas mkl openssl];

  meta = with lib; {
    description = "Software for a single pixel camera";
    homepage = "https://github.com/taktoa/monocle";
    license = with licenses; [ asl20 ];
    maintainers = with maintainers; [ taktoa ];
    mainProgram = "monocle";
  };
}
