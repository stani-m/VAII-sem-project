#!/bin/sh
if [ "$1" = release ]; then
  cargo build --profile deploy
  wasm-bindgen --target web --no-typescript --out-dir ../www/ target/wasm32-unknown-unknown/deploy/game.wasm
  wasm-opt -O --converge -o ../www/game_bg.wasm ../www/game_bg.wasm
else
  cargo build --release
  wasm-bindgen --target web --no-typescript --out-dir ../www/ target/wasm32-unknown-unknown/release/game.wasm
fi
