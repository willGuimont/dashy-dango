cargo build --release
w4 bundle --html build/html/index.html --title "Dashy Dango" --description "Dashy game" --icon-file "assets/sprites/dangoBeeg.png" target/wasm32-unknown-unknown/release/cart.wasm
git add -f build/html && git commit -m "Deploy"
git push origin `git subtree split --prefix build/html`:gh-pages --force
