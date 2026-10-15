#!/usr/bin/env sh
# Compile the Rust crate to WebAssembly and generate JS bindings into the
# Svelte app. Needs: rustup target wasm32-unknown-unknown, and
# wasm-bindgen-cli at the exact version pinned in wasm/Cargo.toml.
set -eu

root="$(cd "$(dirname "$0")/.." && pwd)"

cargo build --manifest-path "$root/wasm/Cargo.toml" --release --target wasm32-unknown-unknown
wasm-bindgen \
  --target web \
  --out-dir "$root/web/src/wasm" \
  "$root/wasm/target/wasm32-unknown-unknown/release/xbox_led.wasm"
