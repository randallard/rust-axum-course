Left off at https://youtu.be/XZtlD_m59sM?si=DeI8er1swsKRdYmP&t=996

## Dev (REPL)

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - To run the tests.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

continued at git@github.com:randallard/rust-web-app.git