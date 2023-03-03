# exhibition_lp

This is a last hackathon LP.

## Usage

For a more thorough explanation of Trunk and its features, please head over to the [repository][trunk].

### Installation

#### Using local rustup and node

必要なツールをインストールする．

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
yarn install
```

### Running

```bash
yarn dev
```

### Release

```bash
yarn release
```
