let
  pkgs = import <nixpkgs> {};

in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      nodejs
      pkg-config
      wasm-pack
    ];
  }
