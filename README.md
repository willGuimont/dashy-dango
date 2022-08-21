# dashy-dango

Dashy Dango a wave fighting game made with [WASM-4](https://wasm4.org/) by [@samX500](https://github.com/samX500) and [@willGuimont](https://github.com/willGuimont).

## Build the cartridge

```bash
cargo build --release
```

## Run the cartridge

```bash
w4 run target/wasm32-unknown-unknown/release/cart.wasm
# or
w4 watch target/wasm32-unknown-unknown/release/cart.wasm
```

## Minify cartridge

Requirements:

- [wasm-opt](https://github.com/WebAssembly/binaryen) <= 97
- [wasm-snip](https://github.com/rustwasm/wasm-snip)

```bash
cargo build --release
./optimize_cart.sh
```

### [bundle](https://wasm4.org/docs/reference/cli#bundle)

```bash
w4 bundle --html build/html/index.html --title Dango --description "Rolling puzzle game" --icon-file "assets/sprites/dangoBeeg.png" build/cart.wasm
w4 bundle --linux dango carts/cart.wasm
```

## Deploy to GitHub-Pages

```bash
./deploy.sh
```

## Useful tools

### [png2src](https://wasm4.org/docs/reference/cli#png2src)

```bash
w4 png2src --rust top.png down.png left.png right.png
```
