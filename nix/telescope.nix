pself: psuper: {
  telescope-rpi = pself.extend (self: super: {
    monocle = self.callPackage ../monocle {
      rustPlatform = self.makeRustPlatform {
        cargo = self.buildPackages.latest.rustChannels.nightly.rust;
        rustc = self.buildPackages.latest.rustChannels.nightly.rust;
      };
    };
    extra_utils = [ self.monocle ];
    initrd_script = ''
      set -x
      #printf "Starting monocle 0"
      #monocle &
      #printf "After monocle 0"
    '';
  });
}
