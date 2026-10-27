# Разработка

## Окружение

Для разработки и сборки требуются Node.js ≥ 24.14, pnpm из поля `packageManager` в `package.json` и Rust с целью WebAssembly. Готовая библиотека поддерживает Node.js ≥ 22.

```sh
rustup target add wasm32-unknown-unknown
pnpm install --frozen-lockfile
```

`wasm-pack` и `wasm-opt` устанавливаются как devDependencies. `wasm-opt` 2 поставляет Binaryen через платформенные optionalDependencies и не запускает install scripts.

## Сборка и проверки

```sh
pnpm run compile
pnpm run optimize
pnpm test
```

`compile` очищает `dist`, собирает Rust → WASM, запускает `prepare.ts` и компилирует TypeScript. `prepublishOnly` выполняет сборку и оптимизацию перед публикацией. Тесты используют собранный `dist`, поэтому после изменений сначала нужна сборка.

В тестах сравниваются все 36 алгоритмов с парами слов и результатов из `snowball-data`. Отдельно проверяются функции языков, фабрика, строковые идентификаторы и `Languages.X`.

## Источники Snowball 3.2.0

- Релиз: https://snowballstem.org/dist/snowball-3.2.0.tar.gz
- SHA-256 архива: `78ec0f192871497b68e11892fd20ef9b26c037c35d1162e65a29cef86111bf67`.
- Полный снимок исходников: [`6389c79f91cd67ed2de3ebdcf84ec8c2d2d23c11`](https://github.com/snowballstem/snowball/tree/6389c79f91cd67ed2de3ebdcf84ec8c2d2d23c11).
- Корпуса: [`019921c3a41c2b70063ee3feb215a00c8e594597`](https://github.com/snowballstem/snowball-data/tree/019921c3a41c2b70063ee3feb215a00c8e594597).

Для сборки компилятора используйте полный снимок: в релизном архиве отсутствует `libstemmer/mkalgorithms.pl`. Файлы компилятора и `.sbl` в указанных источниках совпадают.

```sh
make -j4 snowball
```

Алгоритмы генерируются этим компилятором; вручную сгенерированные файлы не редактируются, кроме адаптации импортов `use snowball::` → `use crate::snowball::`. Runtime берётся из `rust/src/snowball/` того же снимка.

## Добавление алгоритма

1. Возьмите `.sbl` из согласованного стабильного выпуска Snowball и скомпилируйте его:

   ```sh
   /path/to/snowball/snowball /path/to/snowball/algorithms/ukrainian.sbl -rust -o rust-stemmers/src/snowball/algorithms/ukrainian
   ```

2. Адаптируйте импорты и зарегистрируйте модуль в `rust-stemmers/src/snowball/algorithms/mod.rs`, вариант `Languages` и ветку `Stemmer::new()` в `rust-stemmers/src/lib.rs`.
3. Добавьте идентификатор в `src/interfaces/index.ts`, отображение в `src/language-map.ts`, функции в `src/languages.ts` и `src/languages.browser.ts`.
4. Скопируйте `voc.txt` и `output.txt` согласованного корпуса в `test/data/voc_<identifier>.txt` и `res_<identifier>.txt`, сохранив его уведомления `COPYING`. Для файлов `.gz` сначала выполните распаковку.
5. Добавьте корпус в `test/languages.test.ts` и проверки публичных способов вызова в `test/stemmer.test.ts`.
6. Обновите README, changelog, список вариантов и образцы текста в `demo/index.html`.

Для Early Modern English используется идентификатор `earlymodernenglish`; он входит в `Language`, но не в `IsoCode`. Конкретные числовые значения `Languages` не входят в контракт API.

## Подготовка релиза

Текущий релиз — 2.2.0. Версии в `package.json` и `rust-stemmers/Cargo.toml` должны совпадать.

Перед выпуском соберите и оптимизируйте WASM, запустите тесты, создайте архив через `pnpm pack` и проверьте его установку в чистом проекте. В архив должны входить `dist/browser/index_bg.wasm`, браузерный JS, TypeScript-декларации, `LICENSE` и `THIRD_PARTY_NOTICES`. Проверьте импорт и новые алгоритмы в Node.js ≥ 22 и браузере.

`prepare.ts` удаляет служебный `.gitignore`, который создаёт `wasm-pack`: иначе npm может исключить весь `dist/browser/` из архива.

Публикация GitHub Release запускает `.github/workflows/release.yml`, который публикует npm-пакет и прикладывает WASM и архив к релизу. Демо разворачивается отдельным workflow при push в `master`.
