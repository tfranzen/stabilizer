
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    rustup
    probe-rs
  ];

  shellHook = ''
    rustup target add thumbv7em-none-eabihf
    cargo install cargo-binutils
    rustup component add llvm-tools-preview
  '';
}

