cargo build --release
./optimize_cart.sh
du -b target/wasm32-unknown-unknown/release/dashy-dango.wasm
