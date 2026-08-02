# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- **Sesotho** (`st`) stemmer — new in [Snowball 3.1.0](https://snowballstem.org/), brings the total to 34 languages
- Test corpus `voc_st.txt` / `res_st.txt` from [snowball-data](https://github.com/snowballstem/snowball-data)

### Changed
- All stemmers regenerated from **Snowball 3.1.1** (were Snowball 3.0.0)
- Stemming results change for words containing apostrophes and elisions in `Danish`,
  `Estonian`, `Finnish`, `German`, `Italian`, `Lithuanian`, `Norwegian` and `Polish`
- `English` — `skis` stems to `ski` again (regression introduced upstream in Snowball 3.0.0)
- `Danish` — undoubling restricted to consonant pairs that actually occur in Danish
- `Finnish` — illative fallback now removes `-n` when the vowel before `-han`/`-hen`/`-hin`/`-hon`/`-hän`/`-hön`
  does not match; illative form `-hun` removed
- `Lithuanian` — `-er-` is no longer removed before regular suffixes
- Test corpora for `Danish`, `Estonian`, `Finnish`, `French`, `German` and `Norwegian` synced with snowball-data
- Dev dependencies updated: `pnpm 11.19.0`, `@types/node 22.20.1`, `tsx 4.23.2`, `vitest 4.1.10`, `wasm-pack 0.15.0`

### Fixed
- Node.js entry no longer logs `using deprecated parameters for the initialization function` —
  the WASM binary is passed to `init()` as `{ module_or_path }`

### Note
- `Sesotho` is inserted alphabetically into the `Languages` enum, so the numeric values of
  `Spanish` through `Yiddish` shift by one. Code using `Languages.X` or ISO codes is unaffected;
  hardcoded numeric values are not.

---

## [2.0.0] — 2026-05-10

### Added
- **16 new languages** compiled from [Snowball 3.0.1](https://snowballstem.org/) sources:
  `Armenian`, `Basque`, `Catalan`, `Czech`, `Esperanto`, `Estonian`, `Hindi`, `Indonesian`, `Irish`, `Lithuanian`, `Nepali`, `Norwegian`, `Persian`, `Polish`, `Serbian`, `Yiddish`
- **Finnish** added to supported languages in README (was already implemented)
- **Functional API** — three levels of usage:
  - Named language functions: `russian('бегущий')` → `'бег'`
  - Factory: `createStemmer('ru')` returns a reusable `(word: string) => string`
  - Raw: `stem('ru', 'бегущий')`
- **ISO 639-1 code support** — pass `'ru'`, `'en'`, etc. alongside `Languages` enum values
- **Browser support** — conditional exports (`node` / `browser` / `default`); single `--target web` WASM build shared by both environments; WASM initializes automatically on import, no `init()` call needed
- **Interactive demo** at `demo/index.html` — word-by-word stemming diff with 33 language samples
- **GitHub Actions** workflow deploying the demo to GitHub Pages

### Changed
- **Minimum Node.js version is now 22**
- Package is now **pure ESM** (`"type": "module"`) — use `import` instead of `require`
- Entire codebase migrated from JavaScript to **TypeScript 6** (`module: NodeNext`)
- SOLID architecture: single-responsibility modules, dependency injection via `iso-map.ts`
- `wasm-bindgen` updated from `0.2.45` to `0.2.121`
- Rust codebase migrated to **edition 2024** — removed `extern crate`, updated module paths to `crate::` prefix
- Snowball runtime (`SnowballEnv`, `Among`) replaced with official **Snowball 3.0** version: `i32` positions, new `go_in_grouping` / `go_out_grouping` methods, all algorithms recompiled from source
- `opt-level = "s"` added to release profile (optimize for size)
- Test suite migrated to **Vitest**, one test per language (43 tests total)
- `packageManager` field set to `pnpm@11`

### Removed
- Class-based `new Stemmer(Languages.X)` API (replaced by functional API above)
- CommonJS support — package is ESM-only
- `serde` / `serde_derive` dependencies

---

## [1.0.2] — Initial release

### Added
- WebAssembly wrapper around [rust-stemmers](https://github.com/CurrySoftware/rust-stemmers) for use in Node.js
- Supported languages: Arabic, Danish, Dutch, English, Finnish, French, German, Greek, Hungarian, Italian, Portuguese, Romanian, Russian, Spanish, Swedish, Tamil, Turkish
- TypeScript typings
- `Stemmer` class with `Languages` enum API
