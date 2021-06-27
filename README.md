## Development

### Commands

> Make sure you read the requirements sections before executing these commands

Script | Description
--- | ---
`trunk build` | Bundles the website under `dist/` directory
`trunk serve` | Executes Trunk's development server

### Requirements

- Rust

### Setup

This solution makes use of Trunk for development and bundling you must
install `trunk` in your system first.

Follow the documentation available in the official [Trunk](https://github.com/thedodd/trunk)
repository.

Then make sure you have the WASM target installed otherwise
add it to your Rust targets using:

```bash
rustup target add wasm32-unknown-unknown
```
