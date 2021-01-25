rust写web
```

wasm-pack build --target web --out-name wasm --out-dir ./static

cargo install miniserve

miniserve ./static --index index.html


```