{ lib, clangStdenv
, fetchFromGitHub
, rustPlatform
, pkgs
}:

clangStdenv.mkDerivation rec {
  pname = "monocle";
  version = "0.1.0";

  src = lib.cleanSourceWith {
    filter = p: t: !(t == "directory" && baseNameOf p == "target");
    src = lib.cleanSource ./.;
  };

  nativeBuildInputs = [pkgs.pkgconfig];
  buildInputs = [
    pkgs.libdrm
    pkgs.openblas
    pkgs.libuvc
    pkgs.libclang.lib
    pkgs.mesa
    pkgs.shaderc
    pkgs.shaderc.lib
    pkgs.vulkan-loader
    pkgs.vulkan-validation-layers
    pkgs.SDL2
    pkgs.valgrind
    pkgs.renderdoc
    pkgs.gdb
  ];

  VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d:${pkgs.vulkan-tools-lunarg}/etc/vulkan/explicit_layer.d";
  SHADERC_LIB_DIR = "${pkgs.shaderc.lib}/lib";

  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

  meta = with lib; {
    description = "Software for a single pixel camera";
    homepage = "https://github.com/taktoa/monocle";
    license = with licenses; [ asl20 ];
    maintainers = with maintainers; [ taktoa ];
    mainProgram = "monocle";
  };
}
