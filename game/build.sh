#!/bin/sh
if [ "$1" = release ]; then
  cargo build --release
  wasm-bindgen --target web --no-typescript --out-dir ../www/ target/wasm32-unknown-unknown/release/game.wasm
  wasm-opt -O --converge -o ../www/game_bg.wasm ../www/game_bg.wasm
else
  cargo build
  wasm-bindgen --target web --no-typescript --out-dir ../www/ target/wasm32-unknown-unknown/debug/game.wasm
fi
