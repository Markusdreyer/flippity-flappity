# How to compile to wasm

1. Install WASM support: `rustup target add wasm32-unknown-unknown`
2. Install wasm-bindgen: `cargo install wasm-bindgen-cli`
3. Build the project: `cargo build --target wasm32-unknown-unknown --release`
4. Link program to browser: `wasm-bindgen target/wasm32-unknown-unknown/release/flippity-flappity.wasm --out-dir wasm --no-modules --no-typescript`


There are some issues with loading the files when running locally. I haven't (read: bothered to) found a way to solve this, so I just host the index file on a file server. I've found [Surge](https://surge.sh) to be the easiest.