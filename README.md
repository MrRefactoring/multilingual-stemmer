<div align="center">

<h1>multilingual-stemmer</h1>

<p>Word stemming for <strong>34 languages</strong> — <a href="https://snowballstem.org/">Snowball 3.1.1</a> compiled to WebAssembly.<br>Node.js · Browser · TypeScript · Zero dependencies.</p>

<p>
  <a href="https://www.npmjs.com/package/multilingual-stemmer"><img src="https://img.shields.io/npm/v/multilingual-stemmer?style=flat-square" alt="npm" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/npm/l/multilingual-stemmer?style=flat-square" alt="license" /></a>
  <img src="https://img.shields.io/badge/node-%3E%3D22-brightgreen?style=flat-square" alt="node" />
  <a href="#supported-languages"><img src="https://img.shields.io/badge/languages-34-blue?style=flat-square" alt="languages" /></a>
</p>

<p>
  <a href="https://mrrefactoring.github.io/multilingual-stemmer/"><strong>Live Demo →</strong></a>
</p>

</div>

---

```ts
import { english, russian, german } from 'multilingual-stemmer';

english('fruitlessly'); // → 'fruitless'
russian('бегущий');     // → 'бег'
german('laufenden');    // → 'lauf'
```

<div align="center">

| ⚡ Zero config | 🌐 34 languages | 📦 WASM — no native addons | 🔷 TypeScript-first | 🖥 Node + Browser |
|:-:|:-:|:-:|:-:|:-:|

</div>

---

## Install

```sh
npm install multilingual-stemmer
# pnpm add multilingual-stemmer
# yarn add multilingual-stemmer
```

> Requires Node.js ≥ 22. ESM only.

---

## Usage

### Named exports — one function per language

The fastest way to get started. Import the language you need and call it directly.

```ts
import { english, russian, french, german, spanish } from 'multilingual-stemmer';

english('running');    // → 'run'
english('beautiful'); // → 'beauti'
french('mangeons');   // → 'mange'
spanish('corriendo'); // → 'corr'
```

Every function is lazy — the WASM stemmer is created on first call and cached automatically.

### `createStemmer(language)` — reusable function

When you stem many words for the same language, create a stemmer once and reuse it:

```ts
import { createStemmer } from 'multilingual-stemmer';

const stem = createStemmer('en'); // ISO 639-1 code
const words = ['running', 'runs', 'runner', 'ran'];
words.map(stem); // → ['run', 'run', 'runner', 'ran']
```

Also accepts `Languages` enum values:

```ts
import { createStemmer, Languages } from 'multilingual-stemmer';

const stem = createStemmer(Languages.Russian);
stem('бегущий'); // → 'бег'
```

### `stem(language, word)` — one-off

No setup, just call:

```ts
import { stem } from 'multilingual-stemmer';

stem('en', 'fruitlessly'); // → 'fruitless'
stem('ru', 'бегущий');     // → 'бег'
stem('de', 'laufenden');   // → 'lauf'
```

---

## Browser

Works out of the box in Vite, webpack, and any modern bundler. The WASM binary
loads automatically on import — no `init()` call needed.

```ts
import { english } from 'multilingual-stemmer';

// That's it. No setup, no init().
document.getElementById('result').textContent = english(inputWord);
```

---

## TypeScript

Full types are included. The `language` parameter accepts either an ISO 639-1 string
or the `Languages` enum — your choice.

```ts
import { stem, createStemmer, Languages } from 'multilingual-stemmer';
import type { IsoCode, Language } from 'multilingual-stemmer';

// Both are valid:
stem(Languages.English, 'running');
stem('en', 'running');

// Type-safe ISO codes:
const code: IsoCode = 'ru';
const stemFn: (word: string) => string = createStemmer(code);
```

---

## Supported languages

![Arabic](https://img.shields.io/badge/Arabic-ar-4a5568?style=flat-square)
![Armenian](https://img.shields.io/badge/Armenian-hy-4a5568?style=flat-square)
![Basque](https://img.shields.io/badge/Basque-eu-4a5568?style=flat-square)
![Catalan](https://img.shields.io/badge/Catalan-ca-4a5568?style=flat-square)
![Czech](https://img.shields.io/badge/Czech-cs-4a5568?style=flat-square)
![Danish](https://img.shields.io/badge/Danish-da-4a5568?style=flat-square)
![Dutch](https://img.shields.io/badge/Dutch-nl-4a5568?style=flat-square)
![English](https://img.shields.io/badge/English-en-4a5568?style=flat-square)
![Esperanto](https://img.shields.io/badge/Esperanto-eo-4a5568?style=flat-square)
![Estonian](https://img.shields.io/badge/Estonian-et-4a5568?style=flat-square)
![Finnish](https://img.shields.io/badge/Finnish-fi-4a5568?style=flat-square)
![French](https://img.shields.io/badge/French-fr-4a5568?style=flat-square)
![German](https://img.shields.io/badge/German-de-4a5568?style=flat-square)
![Greek](https://img.shields.io/badge/Greek-el-4a5568?style=flat-square)
![Hindi](https://img.shields.io/badge/Hindi-hi-4a5568?style=flat-square)
![Hungarian](https://img.shields.io/badge/Hungarian-hu-4a5568?style=flat-square)
![Indonesian](https://img.shields.io/badge/Indonesian-id-4a5568?style=flat-square)
![Irish](https://img.shields.io/badge/Irish-ga-4a5568?style=flat-square)
![Italian](https://img.shields.io/badge/Italian-it-4a5568?style=flat-square)
![Lithuanian](https://img.shields.io/badge/Lithuanian-lt-4a5568?style=flat-square)
![Nepali](https://img.shields.io/badge/Nepali-ne-4a5568?style=flat-square)
![Norwegian](https://img.shields.io/badge/Norwegian-no-4a5568?style=flat-square)
![Persian](https://img.shields.io/badge/Persian-fa-4a5568?style=flat-square)
![Polish](https://img.shields.io/badge/Polish-pl-4a5568?style=flat-square)
![Portuguese](https://img.shields.io/badge/Portuguese-pt-4a5568?style=flat-square)
![Romanian](https://img.shields.io/badge/Romanian-ro-4a5568?style=flat-square)
![Russian](https://img.shields.io/badge/Russian-ru-4a5568?style=flat-square)
![Serbian](https://img.shields.io/badge/Serbian-sr-4a5568?style=flat-square)
![Sesotho](https://img.shields.io/badge/Sesotho-st-4a5568?style=flat-square)
![Spanish](https://img.shields.io/badge/Spanish-es-4a5568?style=flat-square)
![Swedish](https://img.shields.io/badge/Swedish-sv-4a5568?style=flat-square)
![Tamil](https://img.shields.io/badge/Tamil-ta-4a5568?style=flat-square)
![Turkish](https://img.shields.io/badge/Turkish-tr-4a5568?style=flat-square)
![Yiddish](https://img.shields.io/badge/Yiddish-yi-4a5568?style=flat-square)

---

## How it works

Stemming reduces a word to its root form, which is useful for search indexing,
text analysis, and NLP pipelines. For example, `running`, `runs`, and `runner`
all stem to `run`.

This package compiles the [Snowball](https://snowballstem.org/) stemming algorithms
(written in Rust via [rust-stemmers](https://github.com/CurrySoftware/rust-stemmers))
to WebAssembly. The WASM binary ships inside the npm package — no native addons,
no build step, no external calls.

---

## License

MIT — see [LICENSE](LICENSE).

> Based on [rust-stemmers](https://github.com/CurrySoftware/rust-stemmers) by CurrySoftware (MIT)
> and [Snowball 3.1.1](https://snowballstem.org/) (BSD-3-Clause).
