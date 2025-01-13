{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs; [
      rustc 
      cargo
      gcc
      openssl
      pkg-config
      rust-analyzer
      libpqxx
      postgresql
    ];
}
