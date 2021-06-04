let
  pkgs = import <nixpkgs> { };

in
pkgs.mkShell {
  buildInputs = with pkgs; [
    nodejs
    wasm-pack
  ];
}
