# Pondos

Pondos is a small game wherein you play a trader, which travels between
ancient Greek city states in the Black Sea area, trying to carve out
a living.

## Development Info

### Running on wasm

The game runs in the browser via wasm. Simply do:

- `rustup target install wasm32-unknown-unknown && cargo install --locked trunk`
- `trunk serve`
- Open `http://127.0.0.1:8080/`
