#!/usr/bin/env bash
set -euo pipefail

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.85.0
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown

# Build
cargo build --target wasm32-unknown-unknown --release --locked

# Assemble output directory
mkdir -p web
cp target/wasm32-unknown-unknown/release/cloudcat.wasm web/
cp index.html web/
cp -r assets favicons web/

# Minify JS
npx terser gl.js          -o web/gl.js
npx terser quad-storage.js -o web/quad-storage.js
npx terser sapp_jsutils.js -o web/sapp_jsutils.js