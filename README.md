# Rustick

An encrypted, private todo app.

## Build

In addition to the regular `cargo build`, you'll need to run a second command to create the WASM:

```
wasm-pack build --target web --out-name wasm --out-dir ../server/static
```

## Running the App

Thankfully there are no secondardy steps once the WASM binary is generated!

Just do the normal cargo workflow:
```
cargo run
```