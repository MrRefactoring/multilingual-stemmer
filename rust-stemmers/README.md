# Rust-ядро multilingual-stemmer

Локальная адаптация [rust-stemmers](https://github.com/CurrySoftware/rust-stemmers) с WebAssembly-биндингами. Содержит 36 алгоритмов из Snowball 3.2.0: 35 языков и Early Modern English.

`src/lib.rs` экспортирует `Languages` и `Stemmer` через `wasm-bindgen`. Алгоритмы в `src/snowball/algorithms/` генерируются официальным компилятором Snowball; `src/snowball/` содержит его Rust-runtime.

```rust
use rust_stemmers::{Languages, Stemmer};

let stemmer = Stemmer::new(Languages::Ukrainian);
assert_eq!(stemmer.stem("акварельний"), "акварел");
```

Алгоритмы ожидают слова в нижнем регистре. Числовые значения `Languages` не входят в контракт API.

Источники, сборка и порядок обновления описаны в [CONTRIBUTING.md](../CONTRIBUTING.md). Публичный Node.js / Browser API — в [README.md](../README.md). Уведомления о лицензиях — в [THIRD_PARTY_NOTICES](../THIRD_PARTY_NOTICES).
