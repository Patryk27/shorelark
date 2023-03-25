{
  description = "Simulation of evolution, powered by neural networks, genetic algorithms & high-school math.";

  inputs = {
    naersk = {
      url = "github:nix-community/naersk";
    };

    napalm = {
      url = "github:nix-community/napalm";
    };

    nixpkgs = {
      url = "github:nixos/nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
    };

    utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = { self, naersk, napalm, nixpkgs, rust-overlay, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            rust-overlay.overlays.default
          ];
        };

        ### ========= ###
        ### Stage 1/2 ###
        #
        # Rust -> WebAssembly

        rust = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        naersk' = pkgs.callPackage naersk {
          cargo = rust;
          rustc = rust;
        };

        shorelark-wasm' = naersk'.buildPackage {
          src = ./.;
          copyLibs = true;
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        };

        # Normally this would be generated via wasm-pack, but since we cannot
        # invoke it from inside naersk yet, we've gotta build the manifest by
        # hand:
        shorelark-wasm-manifest = pkgs.writeText "package.json" ''
          {
            "name": "lib-simulation-wasm",
            "module": "lib_simulation_wasm.js",
            "types": "lib_simulation_wasm.d.ts"
          }
        '';

        # Optimizes the WebAssembly file, joins it together with `package.json`
        # and builds the TypeScript definitions our frontend code needs.
        #
        # As before, usually this is done by wasm-pack.
        shorelark-wasm = pkgs.runCommand "shorelark-wasm" { } ''
          mkdir $out
          cd $out

          ${pkgs.binaryen}/bin/wasm-opt \
              --strip-debug \
              -O4 \
              ${shorelark-wasm'}/lib/lib_simulation_wasm.wasm \
              -o lib_simulation_wasm.wasm

          cp ${shorelark-wasm-manifest} package.json

          ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              lib_simulation_wasm.wasm \
              --out-dir .
        '';

        ### ========= ###
        ### Stage 2/2 ###
        #
        # WebAssembly + HTML + CSS = <3

        napalm' = pkgs.callPackage napalm { };

        shorelark-www = napalm'.buildPackage ./www {
          installPhase = ''
            # Inside `package.json`, our `lib-simulation-wasm` is included via
            # `file:../libs` - this causes `napalm` to create a broken symlink
            # (as `../libs` doesn't exist here anymore), which we have to fix:
            rm node_modules/lib-simulation-wasm
            ln -s ${shorelark-wasm} node_modules/lib-simulation-wasm

            npm run build

            mv dist $out
          '';
        };

      in
      {
        defaultPackage = shorelark-www;
      }
    );
}
