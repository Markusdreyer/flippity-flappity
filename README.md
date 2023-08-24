# How to compile to wasm

1. Install WASM support: `rustup target add wasm32-unknown-unknown`
2. Install wasm-bindgen: `cargo install wasm-bindgen-cli`
3. Build the project: `cargo build --target wasm32-unknown-unknown --release`
4. Link program to browser: `wasm-bindgen target/wasm32-unknown-unknown/release/flippity-flappity.wasm --out-dir wasm --no-modules --no-typescript`
5. Host game with any http server: `python -m http.server --directory wasm`

Or use a service such as [Surge](https://surge.sh) to get a public link.
