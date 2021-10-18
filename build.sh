#!/bin/sh
if [ "$1" = release ]; then
  cargo build --release --target wasm32-unknown-unknown
  wasm-bindgen --target web --no-typescript --out-dir www/ target/wasm32-unknown-unknown/release/vaii_sem_project.wasm
  wasm-opt -O --converge -o www/vaii_sem_project_bg.wasm www/vaii_sem_project_bg.wasm
else
  cargo build --target wasm32-unknown-unknown
  wasm-bindgen --target web --no-typescript --out-dir www/ target/wasm32-unknown-unknown/debug/vaii_sem_project.wasm
fi
