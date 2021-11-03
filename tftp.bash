#!/usr/bin/env bash

atftpd="$(nix-build --no-out-link . -A atftp)/bin/atftpd"
sudo ip link set dev enp38s0f1 up || true
sudo ip addr add 192.168.3.1/32 dev enp38s0f1 || true
sudo ip route add 192.168.3.0/24 dev enp38s0f1 || true
sudo "${atftpd}" --daemon --no-fork --user root --group users --logfile - --bind-address 0.0.0.0 -v7 /home/remy/tftp
