build-game:
  cargo build --target wasm32-unknown-unknown --release

build-wasm: build-game
  wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name mousy ./target/wasm32-unknown-unknown/release/motion_smoothing.wasm 

