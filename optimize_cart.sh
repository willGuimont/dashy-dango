#!/usr/bin/env bash
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm -o target/wasm32-unknown-unknown/release/dashy-dango.wasm
wasm-opt -Oz --converge --strip-producers --strip-debug --dce --strip-dwarf --strip-target-features -all --ignore-implicit-traps --const-hoisting target/wasm32-unknown-unknown/release/dashy-dango.wasm -o target/wasm32-unknown-unknown/release/dashy-dango.wasm
