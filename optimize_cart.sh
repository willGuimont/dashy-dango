#!/usr/bin/env bash
cp target/wasm32-unknown-unknown/release/cart.wasm target/wasm32-unknown-unknown/release/dashy-dango.wasm
# TODO maybe add --snip-rust-fmt-code if we don't use format!
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code --skip-producers-section target/wasm32-unknown-unknown/release/cart.wasm -o target/wasm32-unknown-unknown/release/dashy-dango.wasm
wasm-opt -Oz --converge --strip-producers --strip-debug --dce --strip-dwarf --strip-target-features -all --ignore-implicit-traps --const-hoisting target/wasm32-unknown-unknown/release/dashy-dango.wasm -o target/wasm32-unknown-unknown/release/dashy-dango.wasm
