cargo build --release
wasm-opt -Oz --converge --strip-producers --strip-debug --strip-dwarf --strip-target-features -all --ignore-implicit-traps --const-hoisting target/wasm32-unknown-unknown/release/cart.wasm -o build/cart.wasm
w4 bundle --html build/html/index.html --title "Dashy Dango" --description "Dashy game" --icon-file "assets/sprites/dangoBeeg.png" build/cart.wasm
git add -f build/html && git commit -m "Deploy"
git push origin `git subtree split --prefix build/html`:refs/heads/gh-pages --force
