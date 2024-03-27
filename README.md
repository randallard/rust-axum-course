Started with Rust Axum Full Course - Web Development ( https://youtu.be/XZtlD_m59sM?si=okL6haG303i-ZSym )

## Dev (REPL)

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - To run the tests.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```