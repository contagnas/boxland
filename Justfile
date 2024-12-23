[working-directory: 'game']
build-game:
  cargo build --target wasm32-unknown-unknown --release

[working-directory: 'game']
build-wasm: build-game
  wasm-bindgen --target web --out-dir ./public/ --out-name game ./target/wasm32-unknown-unknown/release/motion_smoothing.wasm 

[working-directory: 'server']
run-server:
  cargo run

