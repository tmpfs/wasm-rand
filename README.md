# RNG in WASM

Demo for generating random values in WASM using `rand` and `getrandom`.

The `main` branch tests using the `0.8` version of `rand` and the `rand-0.6.5` branch tests using `rand@0.6.5` with the `wasm-bindgen` feature enabled.

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
