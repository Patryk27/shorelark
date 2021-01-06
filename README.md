# Shorelark

Simulation of life & evolution - powered by neural networks, genetic algorithms, and high-school math.

I've described the implementation in detail on my blog: [Learning to Fly](https://pwy.io/en/posts/learning-to-fly-pt1).

# Building

```bash
$ cd libs/simulation-wasm
$ wasm-pack build --release
$ cd ../../www
$ npm install
$ npm run start
```

# License

Copyright (c) 2020, Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
