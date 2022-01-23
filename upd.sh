#!/bin/bash
cargo build --release; 
cp target/wasm32-unknown-unknown/release/cart.wasm ./cart.wasm; 
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code -o optimized.wasm cart.wasm;
wasm-opt -O3 --strip-producers --strip-debug --dce --zero-filled-memory optimized.wasm -o optimized.wasm;
stat ./optimized.wasm | rg Size;
cp -f ./optimized.wasm ./space_delivery.wasm
w4 bundle optimized.wasm --title "Space delivery" --linux space_delivery --html space_delivery.html --windows space_delivery.exe --mac space_delivery_mac; 
# ./space_delivery
