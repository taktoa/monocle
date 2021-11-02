pself: psuper: {
  telescope-rpi = pself.extend (self: super: {
    monocle = self.callPackage ../monocle {};
    extra_utils = [ self.monocle ];
    initrd_script = ''
      monocle &
      sleep 20
    '';
  });
}
