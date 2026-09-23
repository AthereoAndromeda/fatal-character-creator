{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = with pkgs; [
    git
    bacon
    cargo-insta
    cargo-nextest
    cargo-expand
  ];

  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
    # cranelift.enable = true;
    wild.enable = true;
  };

  enterTest = ''
    cargo nextest run
  '';
}
