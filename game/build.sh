#!/bin/sh
if [ "$1" = deploy ]; then
  cargo build --profile deploy
  wasm-bindgen --target web --no-typescript --remove-name-section --remove-producers-section --out-dir ../www/ target/wasm32-unknown-unknown/deploy/game.wasm
else
  cargo build --release
  wasm-bindgen --target web --no-typescript --remove-producers-section --out-dir ../www/ target/wasm32-unknown-unknown/release/game.wasm
fi
