rust写web

安装一下工具：
1. rust
2. cargo
3. Wasm 构建工具
  * wasm-pack
  * wasm-bindgen

```

wasm-pack build --target web --out-name wasm --out-dir ./static

cargo install miniserve

miniserve ./static --index index.html


```