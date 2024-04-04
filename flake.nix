{
  description = "Simulation of evolution, powered by neural networks, genetic algorithms and high school math.";

  inputs = {
    naersk = {
      url = "github:nix-community/naersk";

      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };

    napalm = {
      url = "github:nix-community/napalm";

      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };

    nixpkgs = {
      url = "github:nixos/nixpkgs";

      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";

      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };

    utils = {
      url = "github:numtide/flake-utils";

      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
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

        # nixpkgs has wasm-bindgen-cli v0.2.91, but we need the newer one
        wasmBindgenCli = pkgs.wasm-bindgen-cli.override {
          version = "0.2.92";
          hash = "sha256-1VwY8vQy7soKEgbki4LD+v259751kKxSxmo/gqE6yV0=";
          cargoHash = "sha256-aACJ+lYNEU8FFBs158G1/JG8sc6Rq080PeKCMnwdpH0=";
        };

        rust = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        naersk' = pkgs.callPackage naersk {
          cargo = rust;
          rustc = rust;
        };

        napalm' = pkgs.callPackage napalm {
          #
        };

        # ---

        shorelarkWasm' = naersk'.buildPackage {
          src = ./.;
          copyLibs = true;
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        };

        shorelarkWasmManifest = pkgs.writeText "package.json" ''
          {
            "name": "lib-simulation-wasm",
            "module": "lib_simulation_wasm.js",
            "types": "lib_simulation_wasm.d.ts"
          }
        '';

        shorelarkWasm = pkgs.runCommand "shorelark-wasm" { } ''
          mkdir $out
          cd $out

          ${pkgs.binaryen}/bin/wasm-opt \
              --strip-debug \
              -O4 \
              ${shorelarkWasm'}/lib/lib_simulation_wasm.wasm \
              -o lib_simulation_wasm.wasm

          cp ${shorelarkWasmManifest} package.json

          ${wasmBindgenCli}/bin/wasm-bindgen \
              lib_simulation_wasm.wasm \
              --out-dir .
        '';

        package = napalm'.buildPackage ./www {
          installPhase = ''
            # Inside `package.json`, our `lib-simulation-wasm` is included via
            # `file:../libs` - this causes `napalm` to create a broken symlink
            # (as `../libs` doesn't exist here anymore), which we have to fix:
            rm node_modules/lib-simulation-wasm
            ln -s ${shorelarkWasm} node_modules/lib-simulation-wasm

            npm run build

            mv dist $out
          '';
        };

      in
      {
        defaultPackage = package;
      }
    );
}
