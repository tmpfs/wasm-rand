# RNG in WASM

Demo for generating random values in WASM via `Crypto.getRandomValues()` (see [webcrypto](https://github.com/w3c/webcrypto/)).

The `main` branch tests using `rand@0.8` with `getrandom@0.2` using multiple threads enabled using [wasm-bindgen-rayon](https://github.com/GoogleChromeLabs/wasm-bindgen-rayon).

For single-threaded versions see the `rand-0.8` and `rand-0.6.5` (older `wasm-bindgen` feature) branches.

## Prerequisites

* `rust@1.54.0`
* `wasm-pack@0.9.1`
* `node@14.17.0`

## Setup

```
make setup
```

## Serve

```
# Compile the webassembly module
make wasm
# Start the client on http://localhost:8080
make client
```

Now visit `http://localhost:8080`.

## Development

During development you should link the WASM module:

```
cd wasm/pkg && yarn link
cd client && yarn link wasm-rand
```
