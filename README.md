<div align="center">

<h1>multilingual-stemmer</h1>

<p>Word stemming for <strong>35 languages and Early Modern English</strong>: 36 <a href="https://snowballstem.org/">Snowball 3.2.0</a> algorithms compiled to WebAssembly.<br>Node.js · Browser · TypeScript · Zero runtime dependencies.</p>

<p>
  <a href="https://www.npmjs.com/package/multilingual-stemmer"><img src="https://img.shields.io/npm/v/multilingual-stemmer?style=flat-square" alt="npm" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/npm/l/multilingual-stemmer?style=flat-square" alt="license" /></a>
  <img src="https://img.shields.io/badge/node-%3E%3D22-brightgreen?style=flat-square" alt="node" />
  <a href="#supported-algorithms"><img src="https://img.shields.io/badge/algorithms-36-blue?style=flat-square" alt="algorithms" /></a>
</p>

<p><a href="https://mrrefactoring.github.io/multilingual-stemmer/"><strong>Live Demo →</strong></a></p>

</div>

```ts
import { english, ukrainian, earlyModernEnglish } from 'multilingual-stemmer';

english('running');
ukrainian('книгами');
earlyModernEnglish('bringeth');
```

Results: `run`, `книг`, `bring`.

## Install

```sh
npm install multilingual-stemmer
```

Requires Node.js ≥ 22. ESM only.

## Usage

### Named language functions

Each algorithm is available as a named function. Its WASM stemmer instance is created on the first call and cached.

```ts
import { english, russian, french } from 'multilingual-stemmer';

english('fruitlessly');
russian('бегущий');
french('mangeons');
```

Results: `fruitless`, `бег`, `mange`.

### `createStemmer(language)`

The factory returns a reusable function for stemming words with the selected algorithm.

```ts
import { createStemmer, Languages } from 'multilingual-stemmer';

const stemUkrainian = createStemmer('uk');
const stemHistoricalEnglish = createStemmer('earlymodernenglish');
const stemEnglish = createStemmer(Languages.English);

stemUkrainian('акварельний');
stemHistoricalEnglish('bringeth');
stemEnglish('running');
```

Results: `акварел`, `bring`, `run`. Early Modern English is also available through `Languages.EarlyModernEnglish`.

### `stem(language, word)`

```ts
import { stem, Languages } from 'multilingual-stemmer';

stem('uk', 'книгами');
stem('earlymodernenglish', 'bringeth');
stem(Languages.Ukrainian, 'акварельний');
```

Select an algorithm using an ISO 639-1 code, a `Languages.X` member, or the `earlymodernenglish` identifier for historical English. Numeric enum values are implementation details and are not guaranteed to remain stable between versions. Use string identifiers when saving configuration.

Algorithms expect lowercase words. Tokenization and case normalization are the caller’s responsibility.

## Browser

The `browser` export condition selects the browser entry point. WASM initializes automatically on import through top-level await; no separate `init()` call is needed. Your bundler must support ESM, top-level await, and `.wasm` assets referenced with `new URL(..., import.meta.url)`.

```ts
import { ukrainian, earlyModernEnglish } from 'multilingual-stemmer';

const result = ukrainian('книгами');
const historicalResult = earlyModernEnglish('bringeth');
```

## TypeScript

Type declarations are included. `IsoCode` contains the ISO 639-1 codes of 35 languages; `Language` also accepts `Languages.X` members and `earlymodernenglish`.

```ts
import { createStemmer } from 'multilingual-stemmer';
import type { IsoCode, Language } from 'multilingual-stemmer';

const code: IsoCode = 'uk';
const variant: Language = 'earlymodernenglish';
const stem = createStemmer(variant);
```

## Supported algorithms

| Language / variant | String identifier | Function |
|---|---|---|
| Arabic | `ar` | `arabic` |
| Armenian | `hy` | `armenian` |
| Basque | `eu` | `basque` |
| Catalan | `ca` | `catalan` |
| Czech | `cs` | `czech` |
| Danish | `da` | `danish` |
| Dutch | `nl` | `dutch` |
| Early Modern English | `earlymodernenglish` | `earlyModernEnglish` |
| English | `en` | `english` |
| Esperanto | `eo` | `esperanto` |
| Estonian | `et` | `estonian` |
| Finnish | `fi` | `finnish` |
| French | `fr` | `french` |
| German | `de` | `german` |
| Greek | `el` | `greek` |
| Hindi | `hi` | `hindi` |
| Hungarian | `hu` | `hungarian` |
| Indonesian | `id` | `indonesian` |
| Irish | `ga` | `irish` |
| Italian | `it` | `italian` |
| Lithuanian | `lt` | `lithuanian` |
| Nepali | `ne` | `nepali` |
| Norwegian | `no` | `norwegian` |
| Persian | `fa` | `persian` |
| Polish | `pl` | `polish` |
| Portuguese | `pt` | `portuguese` |
| Romanian | `ro` | `romanian` |
| Russian | `ru` | `russian` |
| Serbian | `sr` | `serbian` |
| Sesotho | `st` | `sesotho` |
| Spanish | `es` | `spanish` |
| Swedish | `sv` | `swedish` |
| Tamil | `ta` | `tamil` |
| Turkish | `tr` | `turkish` |
| Ukrainian | `uk` | `ukrainian` |
| Yiddish | `yi` | `yiddish` |

Early Modern English is a separate historical variant. Use `en`, `Languages.English`, or `english` for modern English.

## Upgrading to 2.2.0

Snowball 3.2.0 removes the French colloquial elision `z'`, changing the stems of affected words. Rebuild affected French search indexes so documents and queries use the same algorithm version.

See [CHANGELOG.md](CHANGELOG.md) for the full release history.

## How it works

Snowball algorithm sources are compiled to Rust and then to WebAssembly with `wasm-pack`. TypeScript 7 compiles the wrapper that provides language functions and the stemmer factory for Node.js and browsers. The npm package includes the WASM binary. Development requires Node.js ≥ 24.14; the published library supports Node.js ≥ 22.

Stemming maps different word forms to a common stem for search and text analysis. The stem is not necessarily a dictionary word; stemming is different from lemmatization.

## License

The project’s own code is licensed under MIT; see [LICENSE](LICENSE). It uses [rust-stemmers](https://github.com/CurrySoftware/rust-stemmers) and Snowball 3.2.0. Full third-party copyright notices and license terms are included in [THIRD_PARTY_NOTICES](THIRD_PARTY_NOTICES), which ships with the npm package.
