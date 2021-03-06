let
  sources = import ./sources.nix;
  nativePkgs = import sources.nixpkgs {};
in
self: super: {
  inherit sources;
  baseFirmware = self.runCommand "base-firmware" {} ''
    mkdir $out
    cp -r ${self.sources.firmware}/boot/{*.dtb,kernel*img,fixup*dat,start*elf,overlays} $out/
  '';
  modulesForKernel = kernel: self.runCommand "modules" {} ''
    mkdir -pv $out/lib/modules/
    cp -r ${self.sources.firmware}/modules/${kernel} $out/lib/modules/
  '';
  libftdi = null;
  atftp = super.atftp.override { gcc = null; readline = null; };
  openocd = super.openocd.overrideAttrs (old: {
    src = self.fetchFromGitHub {
      owner = "raspberrypi";
      repo = "openocd";
      rev = "14c0d0d330bd6b2cdc0605ee9a9256e5627a905e";
      fetchSubmodules = true;
      sha256 = "sha256-o7shTToj6K37Xw+Crwif5WwB4GfPYIiMJ/o/9u3xrsE=";
    };
    nativeBuildInputs = old.nativeBuildInputs ++ [
      nativePkgs.autoreconfHook
      nativePkgs.gcc
    ];
    #buildInputs = old.buildInputs ++ [ self.tcl ];
    preConfigure = ''
      pwd
      ls -l
    '';
    configureFlags = [
      "--enable-bcm2835gpio"
      "--enable-sysfsgpio"
    ];
  });
  shrunken_busybox = self.runCommand "shrunk-busybox" {
    busybox = self.busybox.override { enableStatic = true; };
    nativeBuildInputs = [ self.buildPackages.nukeReferences ];
  } ''
    mkdir $out
    cp -vir $busybox/bin $out/
    chmod +w $out/bin
    chmod +w $out/bin/busybox
    nuke-refs $out/bin/busybox
  '';
  boottime = self.stdenv.mkDerivation {
    name = "boottime";
    unpackPhase = ''
      cp ${rpi/boottime.c} boottime.c
      export sourceRoot=.
    '';
    buildPhase = ''
      $CC boottime.c -o boottime
    '';
    installPhase = ''
      mkdir -p $out/bin
      cp boottime $out/bin/
    '';
  };
  rpi-tools = self.lib.makeScope self.newScope (iself: {
    utils = iself.callPackage rpi/utils {};
    tlsf = null;
    common = iself.callPackage "${self.sources.rpi-open-firmware}/common" {};
  });
  withWifi = false;
  etc = self.runCommand "etc" {
    nsswitch = ''
      passwd:    files systemd
      group:     files systemd
      shadow:    files

      hosts:     files mymachines mdns_minimal [NOTFOUND=return] dns mdns myhostname
      networks:  files

      ethers:    files
      services:  files
      protocols: files
      rpc:       files
    '';
    # sets root password to password
    passwd = ''
      root:nxz2xIegZ0Ytc:0:0:System administrator:/:/bin/sh
      avahi:x:10:10:avahi-daemon privilege separation user:/var/empty:/run/current-system/sw/bin/nologin
      sshd:x:498:65534:SSH privilege separation user:/var/empty:/run/current-system/sw/bin/nologin
      nscd:x:2:2:nscd privilege separation user:/var/empty:/run/current-system/sw/bin/nologin
    '';
    group = ''
      avahi:x:10:
    '';
    sshd_config = ''
      UsePAM no
      Port 22
    '';
    nscd = ''
      server-user             nscd

      enable-cache            passwd          yes
      positive-time-to-live   passwd          0
      negative-time-to-live   passwd          0
      shared                  passwd          yes

      enable-cache            group           yes
      positive-time-to-live   group           0
      negative-time-to-live   group           0
      shared                  group           yes

      enable-cache            netgroup        yes
      positive-time-to-live   netgroup        0
      negative-time-to-live   netgroup        0
      shared                  netgroup        yes

      enable-cache            hosts           yes
      positive-time-to-live   hosts           0
      negative-time-to-live   hosts           0
      shared                  hosts           yes

      enable-cache            services        yes
      positive-time-to-live   services        0
      negative-time-to-live   services        0
      shared                  services        yes
    '';
    services = ''
      tftp             69/tcp     # Trivial File Transfer
      tftp             69/udp     # Trivial File Transfer
      ssh              22/tcp     # The Secure Shell (SSH) Protocol
      ssh              22/udp     # The Secure Shell (SSH) Protocol
      ssh              22/sctp    # SSH
    '';
    protocols = ''
      tcp              6 TCP            # Transmission Control
      udp              17 UDP           # User Datagram
      sctp             132 SCTP         # Stream Control Transmission Protocol
    '';

    passAsFile = [
      "nsswitch" "passwd" "sshd_config" "nscd" "group" "services" "protocols"
    ];
    nativeBuildInputs = [ nativePkgs.nukeReferences ];
  } ''
    mkdir -p $out/ssh
    cd $out
    ${self.lib.optionalString self.withWifi ''
      cp ${rpi/wpa_supplicant.conf} wpa_supplicant.conf
    ''}
    cp -r ${self.avahi}/etc/avahi avahi
    chmod +w -R avahi
    for x in avahi/avahi-autoipd.action avahi/avahi-dnsconfd.action; do
      nuke-refs $x
    done
    cp $nsswitchPath nsswitch.conf
    cp $passwdPath passwd
    cp $groupPath group
    cp $sshd_configPath ssh/sshd_config
    cp $nscdPath nscd.conf
    cp $servicesPath services
    cp $protocolsPath protocols
  '';
  # 5.10.81-v7l+
  moduleClosureForKernel = kernel: self.makeModulesClosure {
    kernel = self.modulesForKernel kernel;
    firmware = self.buildEnv {
      name = "all-the-firmware";
      paths = [ self.firmwareLinuxNonfree ];
      ignoreCollisions = true;
    };
    allowMissing = true;
    rootModules = [
      "dwc2"
      "usb_f_acm"
      "usb_f_rndis"
      "usb_f_mass_storage"
      "gadgetfs" # for custom userland gadgets
      "usb_f_hid"
      "usb_f_rndis"
      "vc4"
      "v3d"
    ] ++ self.extra_modules;
  };
  extra_modules = [];
  installedPackages = self.buildEnv {
    name = "bin";
    paths = [
      self.shrunken_busybox
      self.boottime
      #self.rpi-tools.utils
      #self.gdb
      self.shrunkenPackages
    ] ++ self.extra_utils;
  };
  libnl = super.libnl.override { pythonSupport = false; };
  shrunkenPackages = self.runCommandCC "shrunken-packages" { nativeBuildInputs = [ nativePkgs.nukeReferences ]; } ''
    mkdir -p $out/{bin,sbin,lib}
    #cp {self.wpa_supplicant}/bin/wpa_supplicant $out/bin
    #cp {self.avahi}/bin/avahi-daemon $out/bin
    cp ${self.strace}/bin/strace $out/bin
    #cp {self.openssh}/bin/sshd $out/bin
    #cp {self.iproute}/bin/ip $out/bin
    cp ${self.dropbear}/bin/dropbear $out/bin/
    cp ${self.glibcCross.bin}/bin/nscd $out/bin
    #cp {self.smi-test}/bin/smi-test $out/bin
    #cp {self.atftp}/bin/atftp $out/bin
    cp -v ${/home/remy/Downloads/vcdbg} $out/sbin/vcdbg
    chmod +x $out/sbin/vcdbg

    cp ${self.hidapi}/lib/libhidapi-hidraw.so.0 $out/lib
    cp ${self.libusb1}/lib/libusb-1.0.so.0 $out/lib
    cp ${self.glibcCross}/lib/lib{m.so.6,dl.so.2,pthread.so.0,c.so.6,rt.so.1,util.so.1,crypt.so.1,resolv.so.2,nss_files.so.2} $out/lib
    cp ${self.udev}/lib/libudev.so.1 $out/lib
    cp $(cat $(cat $NIX_CC/nix-support/orig-cc)/nix-support/propagated-build-inputs)/armv7l-unknown-linux-gnueabihf/lib/lib{gcc_s.so.1,ssp.so.0} $out/lib/
    cp ${self.utillinux.out}/lib/lib{mount.so.1,blkid.so.1,uuid.so.1} $out/lib/
    cp ${self.openssl.out}/lib/lib{ssl.so.1.1,crypto.so.1.1} $out/lib
    cp ${self.libnl.out}/lib/li{bnl-3.so.200,bnl-genl-3.so.200} $out/lib
    cp ${self.pcsclite.out}/lib/libpcsclite.so.1 $out/lib
    cp ${self.dbus.lib}/lib/libdbus-1.so.3 $out/lib
    cp ${self.systemd}/lib/libsystemd.so.0 $out/lib
    cp ${self.xz.out}/lib/liblzma.so.5 $out/lib
    cp ${self.lz4.out}/lib/liblz4.so.1 $out/lib
    cp ${self.libcap.lib}/lib/libcap.so.2 $out/lib
    cp ${self.libgcrypt.out}/lib/libgcrypt.so.20 $out/lib
    cp ${self.libgpgerror}/lib/libgpg-error.so.0 $out/lib
    #cp {self.avahi}/lib/lib{avahi-common.so.3,avahi-core.so.7} $out/lib
    cp ${self.libdaemon}/lib/libdaemon.so.0 $out/lib
    cp ${self.expat}/lib/libexpat.so.1 $out/lib
    cp ${self.libunwind}/lib/lib{unwind-ptrace.so.0,unwind-arm.so.8,unwind.so.8} $out/lib
    cp ${self.pam}/lib/libpam.so.0 $out/lib
    cp ${self.zlib}/lib/libz.so.1 $out/lib
    cp ${self.libkrb5}/lib/lib{gssapi_krb5.so.2,krb5.so.3,k5crypto.so.3,com_err.so.3,krb5support.so.0} $out/lib
    cp ${self.keyutils.lib}/lib/libkeyutils.so.1 $out/lib

    linker=$(basename $(cat $NIX_CC/nix-support/dynamic-linker))
    chmod +w -R $out

    for x in $out/lib/lib{pthread.so.0,gcc_s.so.1,rt.so.1,mount.so.1,crypto.so.1.0.0,dbus-1.so.3,gpg-error.so.0,ssp.so.0,daemon.so.0,avahi-common.so.3,pam.so.0,util.so.1,z.so.1,crypt.so.1,resolv.so.2,gssapi_krb5.so.2,krb5.so.3,k5crypto.so.3,com_err.so.3,nss_files.so.2}; do
      nuke-refs $x
    done
    nuke-refs $out/bin/avahi-daemon
    nuke-refs $out/bin/sshd
    nuke-refs $out/bin/nscd

    for bin in $out/bin/*; do
      patchelf --set-rpath $out/lib --set-interpreter $out/lib/$linker $bin
    done
    for lib in $out/lib/*; do
      patchelf --set-rpath $out/lib $lib
    done

    cp $(cat $NIX_CC/nix-support/dynamic-linker) $out/lib/$linker
    chmod +w -R $out
    $STRIP $out/lib/$linker
    nuke-refs $out/lib/libc.so*
    nuke-refs $out/lib/libdl.so.2
    nuke-refs $out/lib/libm.so.6
    sed -i -e 's@${self.glibcCross.out}@/nix/store/eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee-${self.glibcCross.name}@' $out/lib/$linker
  '';
  trimRootDir = ''
    rm kernel8.img kernel7.img kernel.img
    rm start.elf start*db.elf start*x.elf start*cd.elf
    rm fixup.dat fixup*db.dat fixup*x.dat fixup*cd.dat
    rm bcm{2708,2710,2709}*dtb
  '';
  kernel_version = "5.10.81";
  kernelVersionList = [
    "-v7l+"
  ];
  kernel_versions = map (x: "${self.kernel_version}${x}") self.kernelVersionList;
  modulesForKernels = self.buildEnv {
    name = "all-the-modules";
    paths = (map self.moduleClosureForKernel self.kernel_versions) ++ [ self.wireless-regdb self.raspberrypiWirelessFirmware ];
    pathsToLink = [ "/lib" ];
    ignoreCollisions = true;
  };
  initrd = self.makeInitrd {
    contents = [
      {
        symlink = "/init";
        object = self.initScript;
      }
      {
        symlink = "/lib/modules";
        object = "${self.modulesForKernels}/lib/modules";
      }
      {
        symlink = "/lib/firmware";
        object = "${self.modulesForKernels}/lib/firmware";
      }
      {
        symlink = "/bin";
        object = "${self.installedPackages}/bin";
      }
      {
        symlink = "/etc";
        object = self.etc;
      }
    ];
  };
  closure = self.runCommand "closure-helper" {} ''
    mkdir $out
    cd $out
    ln -s ${self.initScript} init
    ln -s ${self.modulesForKernels} modules
    ln -s ${self.installedPackages} bin
    ln -s ${self.etc} etc
  '';
  # see also:
  # https://elinux.org/images/e/ef/USB_Gadget_Configfs_API_0.pdf
  initScript = self.writeTextFile {
    name = "init";
    text = ''
      #!/bin/ash

      set -x
      mount -t proc proc proc
      mount -t sysfs sys sys
      mount -t devtmpfs dev dev
      mount -t configfs none /sys/kernel/config
      mkdir /dev/pts
      mount -t devpts devpts /dev/pts
      mount -t debugfs debugfs /sys/kernel/debug

      exec > /dev/tty1 2>&1 < /dev/tty1

      depmod
      export serial=$(cut -c9-16 < /proc/device-tree/serial-number)
      hostname pi-''${serial}

      ${self.initrd_script}

      exec ash
    '';
    executable = true;
  };
  custom-overlays = self.callPackage ({ runCommand, dtc, }: runCommand "custom-overlays" { nativeBuildInputs = [ dtc ]; } ''
    mkdir -p $out/overlays
    cd $out/overlays
    dtc -@ -Hepapr -I dts -O dtb -o smi-speed.dtbo ${rpi/smi-speed-overlay.dts}
  '') {};
  firmware-with-custom-overlays = self.buildEnv {
    name = "firmware-with-custom-overlays";
    paths = [ self.baseFirmware self.custom-overlays ];
  };
  rootDir = self.runCommand "rootdir" {} ''
    mkdir $out
    cd $out
    ln -s ${self.firmware-with-custom-overlays}/* .
    ln -s ${self.initrd}/initrd initrd
    ln -s ${self.monocle}/bin/raspi raspi
    cp ${./modulator.edid} modulator.edid
    ls -lLhs initrd
    cat <<EOF > config.txt
    dtoverlay=dwc2
    dtoverlay=smi
    dtoverlay=smi-dev
    dtoverlay=smi-speed
    dtoverlay=vc4-fkms-v3d
    dtparam=axiperf
    enable_uart=1
    uart_2ndstage=1
    dtoverlay=disable-bt
    hdmi_edid_file=1
    hdmi_edid_filename:1=modulator.edid
    #hdmi_group=2
    #hdmi_enable_4kp60=1
    hdmi_force_hotplug:1=1
    disable_overscan=1

    hdmi_pixel_freq_limit:1=400000000
    hdmi_timings:1=1440 0 100 20 100 2560 0 20 2 24 0 0 0 40 0 173040000 0
    hdmi_drive:1=2
    hdmi_group:1=2
    hdmi_mode:1=87
    hdmi_force_mode:1=1
    #disable_overscan=1
    max_framebuffer_width=2560
    max_framebuffer_height=2560
    display_rotate=0
    framebuffer_width=1440
    framebuffer_height=2560

    initramfs initrd followkernel
    EOF

    cat <<EOF > cmdline.txt
    nada console=tty1 console=serial0,115200 iomem=relaxed
    EOF

    ${self.trimRootDir}
  '';
  rootZip = self.runCommand "rootzip" { nativeBuildInputs = [ self.buildPackages.zip ]; } ''
    cd ${self.rootDir}
    mkdir $out
    zip -r $out/root.zip *
    cd $out
    mkdir nix-support
    echo "file binary-dist $out/root.zip" > nix-support/hydra-build-products
  '';
  # monocle = self.callPackage ../monocle {
  #   rustPlatform = self.makeRustPlatform {
  #     cargo = (self.buildPackages.rustChannelOf { date = "2021-11-15"; channel = "nightly"; }).rust;
  #     rustc = (self.buildPackages.rustChannelOf { date = "2021-11-15"; channel = "nightly"; }).rust;
  #   };
  # };
  #monocle = self.naersk.buildPackage {
  #  name = "monocle";
  #  version = "0.1.0";
  #  src = self.lib.cleanSourceWith {
  #    filter = p: t: !(t == "directory" && baseNameOf p == "target");
  #    src = self.lib.cleanSource ../monocle;
  #  };
  #  buildInputs = [
  #    self.libdrm
  #  ];
  #  CARGO_BUILD_TARGET = self.rust.toRustTargetSpec self.stdenv.hostPlatform;
  #  #singleStep = true;
  #};
  extra_utils = [
    self.atftp
    self.dtc
  ];
  initrd_script = ''
    set -x
    ip link set dev eth0 up
    ip addr add 192.168.3.2/32 dev eth0
    ip route add 192.168.3.0/24 dev eth0
    mkdir -pv /etc/dropbear /var/log
    touch /var/log/lastlog
    /bin/dropbear -R -E &
    modprobe vc4
    #modprobe v3d
    while test ! -f bin/raspi; do
        atftp -g -r "$serial/raspi" -l bin/raspi 192.168.3.1
    done
    chmod +x bin/raspi
    mkdir -pv /mnt
    mount /dev/sda1 /mnt
    export RUST_BACKTRACE=full
    printf "\n[------- REBOOT -------]\n" >> /mnt/log.txt
    /bin/raspi >> /mnt/log.txt 2>&1 &
    #atftp -p -l /calibration.txt 192.168.3.1
  '';
}
