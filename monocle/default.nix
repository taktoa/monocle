{ lib, stdenv
, fetchFromGitHub
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "monocle";
  version = "0.1.0";

  src = lib.cleanSource ./.;

  cargoSha256 = "0h9m5qfznpd1p9k1pxd00qldd995gbz9pk3pxq5n408jrhl7zcw4";

  nativeBuildInputs = [];

  meta = with lib; {
    description = "Software for a single pixel camera";
    homepage = "https://github.com/taktoa/monocle";
    license = with licenses; [ asl20 ];
    maintainers = with maintainers; [ taktoa ];
    mainProgram = "monocle";
  };
}
