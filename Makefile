build:
	cargo build --target wasm32-unknown-unknown --release

bind: build
	wasm-bindgen target/wasm32-unknown-unknown/release/flippity-flappity.wasm \
		--out-dir wasm --no-modules --no-typescript

serve: bind
	python -m http.server --directory wasm
