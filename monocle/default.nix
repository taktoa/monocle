{ lib, stdenv
, fetchFromGitHub
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "monocle";
  version = "0.1.0";

  src = lib.cleanSource ./.;

  cargoSha256 = "08w5j5nc85nq8s4hbi1qij9bsjkwc72jylz36s6kz0mf4k617slr";

  nativeBuildInputs = [];

  meta = with lib; {
    description = "Software for a single pixel camera";
    homepage = "https://github.com/taktoa/monocle";
    license = with licenses; [ asl20 ];
    maintainers = with maintainers; [ taktoa ];
    mainProgram = "monocle";
  };
}
