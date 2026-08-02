# Contributing

## Prerequisites

Install [Rust](https://rustup.rs/) and add the WebAssembly target:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

That's it — `wasm-pack` and `wasm-opt` are installed automatically via `pnpm install`.

## Setup

```sh
pnpm install
```

## Build

```sh
pnpm run compile   # Rust → WebAssembly
node prepare.js    # rewire ESM wrapper
pnpm run optimize  # optimize .wasm binary size
```

Or all at once (as on publish):

```sh
pnpm run prepublishOnly
```

## Test

```sh
pnpm test
```

Tests run against all 34 supported languages using vocabulary/result pairs from [snowball-data](https://github.com/snowballstem/snowball-data).

## Adding a new language

1. Get the Snowball algorithm source (`.sbl`) from [snowballstem/snowball](https://github.com/snowballstem/snowball)
2. Compile it to Rust:
   ```sh
   /path/to/snowball/snowball algorithms/<lang>.sbl -rust -o rust-stemmers/src/snowball/algorithms/<lang>
   ```
3. Fix the generated `use snowball::` imports to `use crate::snowball::` 
4. Add `pub mod <lang>;` to `rust-stemmers/src/snowball/algorithms/mod.rs`
5. Add the language variant to `Languages` enum and `Stemmer::new()` match in `rust-stemmers/src/lib.rs`
6. Add test data (`voc_<iso>.txt`, `res_<iso>.txt`) to `test/data/`
7. Add a `testLanguage(...)` call in `test/index.js`
