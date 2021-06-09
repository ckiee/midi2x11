{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ alsaLib libjack2 xorg.libX11 xorg.libXtst ];
}
