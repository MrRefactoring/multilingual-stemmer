<div align="center">

<h1>multilingual-stemmer</h1>

<p>Стемминг для <strong>35 языков и Early Modern English</strong>: 36 алгоритмов <a href="https://snowballstem.org/">Snowball 3.2.0</a> в WebAssembly.<br>Node.js · Browser · TypeScript · Без runtime-зависимостей.</p>

<p>
  <a href="https://www.npmjs.com/package/multilingual-stemmer"><img src="https://img.shields.io/npm/v/multilingual-stemmer?style=flat-square" alt="npm" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/npm/l/multilingual-stemmer?style=flat-square" alt="license" /></a>
  <img src="https://img.shields.io/badge/node-%3E%3D22-brightgreen?style=flat-square" alt="node" />
  <a href="#поддерживаемые-алгоритмы"><img src="https://img.shields.io/badge/algorithms-36-blue?style=flat-square" alt="algorithms" /></a>
</p>

<p><a href="https://mrrefactoring.github.io/multilingual-stemmer/"><strong>Интерактивное демо →</strong></a></p>

</div>

```ts
import { english, ukrainian, earlyModernEnglish } from 'multilingual-stemmer';

english('running');
ukrainian('книгами');
earlyModernEnglish('bringeth');
```

Результаты: `run`, `книг`, `bring`.

## Установка

```sh
npm install multilingual-stemmer
```

Требуется Node.js ≥ 22. Пакет использует ESM.

## Использование

### Функции языков

Каждый алгоритм доступен как отдельная функция. Экземпляр WASM-стеммера создаётся при первом вызове и кешируется.

```ts
import { english, russian, french } from 'multilingual-stemmer';

english('fruitlessly');
russian('бегущий');
french('mangeons');
```

Результаты: `fruitless`, `бег`, `mange`.

### `createStemmer(language)`

Фабрика возвращает функцию для многократной обработки слов.

```ts
import { createStemmer, Languages } from 'multilingual-stemmer';

const stemUkrainian = createStemmer('uk');
const stemHistoricalEnglish = createStemmer('earlymodernenglish');
const stemEnglish = createStemmer(Languages.English);

stemUkrainian('акварельний');
stemHistoricalEnglish('bringeth');
stemEnglish('running');
```

Результаты: `акварел`, `bring`, `run`. Для Early Modern English также доступен `Languages.EarlyModernEnglish`.

### `stem(language, word)`

```ts
import { stem, Languages } from 'multilingual-stemmer';

stem('uk', 'книгами');
stem('earlymodernenglish', 'bringeth');
stem(Languages.Ukrainian, 'акварельний');
```

Язык выбирается ISO 639-1 кодом, именем `Languages.X` или идентификатором `earlymodernenglish` для исторического английского. Числовые значения enum — деталь реализации; их стабильность между версиями не гарантируется. Для сохранения настроек используйте строковые идентификаторы.

Алгоритмы ожидают слова в нижнем регистре. Разбиение текста на слова и нормализацию регистра выполняет вызывающий код.

## Браузер

Браузерная точка входа выбирается условием `browser` в `exports`. WASM загружается при импорте через top-level await, отдельный вызов `init()` не нужен. Сборщик должен поддерживать ESM, top-level await и загрузку файла `.wasm` через `new URL(..., import.meta.url)`.

```ts
import { ukrainian, earlyModernEnglish } from 'multilingual-stemmer';

const result = ukrainian('книгами');
const historicalResult = earlyModernEnglish('bringeth');
```

## TypeScript

Типы входят в пакет. `IsoCode` содержит ISO 639-1 коды 35 языков, а `Language` также принимает `Languages.X` и `earlymodernenglish`.

```ts
import { createStemmer } from 'multilingual-stemmer';
import type { IsoCode, Language } from 'multilingual-stemmer';

const code: IsoCode = 'uk';
const variant: Language = 'earlymodernenglish';
const stem = createStemmer(variant);
```

## Поддерживаемые алгоритмы

| Язык / вариант | Строковый идентификатор | Функция |
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

Early Modern English — отдельный исторический вариант английского. Современный английский выбирается через `en`, `Languages.English` или `english`.

## Изменения при обновлении до 2.2.0

Snowball 3.2.0 удаляет французскую разговорную элизию `z'`. Результаты обработки таких слов изменились. Пересоздайте затронутые французские поисковые индексы, чтобы документы и запросы обрабатывались одной версией алгоритма.

Полный список изменений находится в [CHANGELOG.md](CHANGELOG.md).

## Устройство библиотеки

Исходники алгоритмов Snowball компилируются в Rust, затем через `wasm-pack` — в WebAssembly. TypeScript 7 компилирует обёртку с функциями языков и фабрикой стеммеров для Node.js и браузера. WASM входит в npm-пакет. Для разработки нужна Node.js ≥ 24.14; установка готовой библиотеки требует Node.js ≥ 22.

Стемминг приводит формы слова к общей основе для поиска и анализа текста. Основа может не быть словарным словом; это не лемматизация.

## Лицензия

Собственный код — MIT, см. [LICENSE](LICENSE). Библиотека использует [rust-stemmers](https://github.com/CurrySoftware/rust-stemmers) и Snowball 3.2.0. Полные условия и copyright сторонних компонентов включены в [THIRD_PARTY_NOTICES](THIRD_PARTY_NOTICES), который поставляется с npm-пакетом.
