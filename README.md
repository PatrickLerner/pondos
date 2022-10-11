# Pondos

[![CI](https://github.com/PatrickLerner/pondos/actions/workflows/ci.yml/badge.svg)](https://github.com/PatrickLerner/pondos/actions/workflows/ci.yml)
![Version](https://img.shields.io/github/v/tag/PatrickLerner/pondos?label=version&color=blue)
![License](https://img.shields.io/github/license/PatrickLerner/pondos?color=blue)
[![pondos.app](https://img.shields.io/website?down_color=lightgrey&down_message=offline&label=web%20version&up_color=blue&up_message=online&url=https%3A%2F%2Fpondos.app%2F)](https://pondos.app/)

Pondos is a small game wherein you play a trader, who travels between
ancient Greek city states in the Black Sea area, trying to carve out
a living.

## Play Online

- https://pondos.app/

## Development Info

### Running on wasm

The game runs in the browser via wasm. Simply do:

- `rustup target install wasm32-unknown-unknown && cargo install --locked trunk`
- `trunk serve`
- Open `http://127.0.0.1:8080/`
