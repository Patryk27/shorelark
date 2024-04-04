# 🐦 Shorelark

Simulation of evolution, powered by neural networks, genetic algorithms and
high school math:

![screenshot](./readme/screenshot.png)

Feeling curious? I've described the implementation _ab ovo_ on my blog:
[Learning to Fly](https://pwy.io/posts/learning-to-fly-pt1).

## Building

### Using Cargo and npm

Requires `cargo`, `npm` and `wasm-pack` (0.11.0):

```bash
# Clone the repository
$ git clone https://github.com/Patryk27/shorelark
$ cd shorelark

# Build Rust code
$ cd libs/simulation-wasm
$ wasm-pack build --release

# Build TypeScript code
$ cd ../../www
$ npm install

# Start the application
$ npm run start

# Now simply open `http://localhost:8080` in your web browser :-)
```

### Using Nix

```bash
# Clone the repository
$ git clone https://github.com/Patryk27/shorelark
$ cd shorelark

# Build the application
$ nix build

# Start the application
$ nix run nixpkgs#php -- -S localhost:8080 -t result

# Now simply open `http://localhost:8080` in your web browser :-)
```

## Usage

Shorelark contains an in-game introduction - just read what the terminal on the
left side says and have fun!

## License

Copyright (c) 2020 Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
