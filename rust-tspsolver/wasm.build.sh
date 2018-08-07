#!/bin/bash

cargo +nightly build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_tspsolver.wasm --out-dir $1 --nodejs
