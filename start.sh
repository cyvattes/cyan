#!/bin/zsh
set -eu

cargo build --release --lib --package cyan_pwa --target wasm32-wasi
wasm-bindgen target/wasm32-wasi/release/cyan_pwa.wasm \
--out-dir cyan_web --no-modules --no-typescript

cd cyan_web
basic-http-server --addr 127.0.0.1:51440 .