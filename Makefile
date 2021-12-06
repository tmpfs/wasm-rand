all: wasm client

wasm:
	@cd wasm && wasm-pack build --target web

setup: wasm
	@cd client && yarn install

client:
	@cd client && yarn start

.PHONY: all wasm setup client
