# 🐦 Shorelark

Simulation of life & evolution - powered by neural networks, genetic algorithms, and high-school math.

I've described the implementation in detail on my blog: [Learning to Fly](https://pwy.io/en/posts/learning-to-fly-pt1).

# Building

```bash
$ cd libs/simulation-wasm
$ wasm-pack build --release
$ cd ../../www
$ npm install
$ npm run start

# Now open your browser and navigate to `localhost:8080`
# (or whatever address npm says it's listening at)
```

# Playing

There's an in-game terminal in which you can enter commands - if you just want to see the birds 
learning, launch the `t` command (as in "train") - i.e. write `t`, press enter, write `t`, press 
enter etc. for a few times and compare what you see.

# License

Copyright (c) 2020-2021, Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
