# Setup

## Install the following Rust tools:
```
$ cargo install wasm-pack          # Compile Rust to Wasm and generate JS interop code
$ cargo install cargo-make         # Task runner
$ cargo install simple-http-server # Simple server to serve assets
```

## Start the build task
```
$ cargo make build
```

That should watch for source changes and rebuild. Note: Seems not to work well under WSL.

## Start the serve task in a new terminal and load http://localhost:8081 in your browser
```
$ cargo make serve
```


