# RNG in WASM

Demo for generating random values in WASM using `rand` and `getrandom`.

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
