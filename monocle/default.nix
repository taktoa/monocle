{ lib, stdenv
, fetchFromGitHub
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "monocle";
  version = "0.1.0";

  src = lib.cleanSource ./.;

  cargoSha256 = "13psc1dw47a4rpff4y2r5bc7h2p10j2ml69zpp1g7vwjq82g70lq";

  nativeBuildInputs = [];

  meta = with lib; {
    description = "Software for a single pixel camera";
    homepage = "https://github.com/taktoa/monocle";
    license = with licenses; [ asl20 ];
    maintainers = with maintainers; [ taktoa ];
    mainProgram = "monocle";
  };
}
