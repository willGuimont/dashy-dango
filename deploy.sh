#!/usr/bin/env bash
cargo build --release
./optimize_cart.sh
cp target/wasm32-unknown-unknown/release/dashy-dango.wasm build/cart.wasm
w4 bundle --html build/html/index.html --title "Dashy Dango" --description "Dashy game" --icon-file "assets/sprites/dangoBeeg.png" build/cart.wasm
git add -f build/html && git commit -m "Deploy"
git push origin $(git subtree split --prefix build/html):refs/heads/gh-pages --force
