# Contributing

## Prerequisites

Development and builds require Node.js ≥ 24.14, the pnpm version pinned in `package.json`, and Rust with the WebAssembly target. The published library supports Node.js ≥ 22.

```sh
rustup target add wasm32-unknown-unknown
pnpm install --frozen-lockfile
```

`wasm-pack` and `wasm-opt` are devDependencies. `wasm-opt` 2 provides Binaryen through platform-specific optionalDependencies and does not run install scripts.

## Build and test

```sh
pnpm run compile
pnpm run optimize
pnpm test
```

`compile` clears `dist`, builds Rust → WASM, runs `prepare.ts`, and compiles TypeScript. `prepublishOnly` builds and optimizes the package before publication. Tests use the built `dist`, so rebuild after changing the source.

The test suite checks all 36 algorithms against vocabulary/result pairs from `snowball-data`. It also tests named language functions, the factory, string identifiers, and `Languages.X` members.

## Snowball 3.2.0 sources

- Release: https://snowballstem.org/dist/snowball-3.2.0.tar.gz
- Archive SHA-256: `78ec0f192871497b68e11892fd20ef9b26c037c35d1162e65a29cef86111bf67`.
- Full source snapshot: [`6389c79f91cd67ed2de3ebdcf84ec8c2d2d23c11`](https://github.com/snowballstem/snowball/tree/6389c79f91cd67ed2de3ebdcf84ec8c2d2d23c11).
- Corpora: [`019921c3a41c2b70063ee3feb215a00c8e594597`](https://github.com/snowballstem/snowball-data/tree/019921c3a41c2b70063ee3feb215a00c8e594597).

Build the compiler from the full snapshot: the release archive omits `libstemmer/mkalgorithms.pl`. The compiler and `.sbl` files in these sources are identical.

```sh
make -j4 snowball
```

Generate algorithms with this compiler. The only manual adjustment to generated files is changing imports from `use snowball::` to `use crate::snowball::`. Use the runtime from `rust/src/snowball/` in the same snapshot.

## Adding an algorithm

1. Take the `.sbl` source from the selected stable Snowball release and compile it:

   ```sh
   /path/to/snowball/snowball /path/to/snowball/algorithms/ukrainian.sbl -rust -o rust-stemmers/src/snowball/algorithms/ukrainian
   ```

2. Adjust the imports and register the module in `rust-stemmers/src/snowball/algorithms/mod.rs`, the `Languages` variant, and the `Stemmer::new()` match arm in `rust-stemmers/src/lib.rs`.
3. Add the identifier in `src/interfaces/index.ts`, its mapping in `src/language-map.ts`, and named functions in `src/languages.ts` and `src/languages.browser.ts`.
4. Copy `voc.txt` and `output.txt` from the selected corpus to `test/data/voc_<identifier>.txt` and `res_<identifier>.txt`, preserving its `COPYING` notices. Decompress `.gz` files first where needed.
5. Add the corpus to `test/languages.test.ts` and public API checks to `test/stemmer.test.ts`.
6. Update the README, changelog, algorithm list, and text samples in `demo/index.html`.

Early Modern English uses the identifier `earlymodernenglish`. It belongs to `Language`, but not to `IsoCode`. Specific numeric `Languages` values are not part of the API contract.

## Preparing a release

The current release is 2.2.1. Versions in `package.json` and `rust-stemmers/Cargo.toml` must match.

Before publication, build and optimize WASM, run tests, create a tarball with `pnpm pack`, and install it in a clean project. The archive must contain `dist/browser/index_bg.wasm`, the browser JavaScript, TypeScript declarations, `LICENSE`, and `THIRD_PARTY_NOTICES`. Check imports and the new algorithms on Node.js ≥ 22 and in a browser.

`prepare.ts` removes the generated `.gitignore` from wasm-pack output; otherwise npm may omit the entire `dist/browser/` directory from the archive.

Publishing a GitHub Release triggers `.github/workflows/release.yml`, which publishes the npm package and attaches the WASM binary and tarball to the release. A separate workflow deploys the demo on pushes to `master`.
