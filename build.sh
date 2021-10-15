#!/bin/sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir www/ target/wasm32-unknown-unknown/release/wasm_sample.wasm
