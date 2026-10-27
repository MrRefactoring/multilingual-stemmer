import { Stemmer, Languages } from './wasm.browser.js';
import { buildLanguageMap, buildResolver } from './language-map.js';
import type { Language, StemmerInstance } from './interfaces/index.js';

const resolveLanguage = buildResolver(buildLanguageMap(Languages));
const cache = new Map<number, StemmerInstance>();

export { Languages };

export const getInstance = (language: Language): StemmerInstance => {
  const key = resolveLanguage(language);
  if (!cache.has(key)) cache.set(key, new Stemmer(key));
  return cache.get(key)!;
};

export const createStemmer = (language: Language): (word: string) => string => {
  const instance = new Stemmer(resolveLanguage(language));
  return (word: string) => instance.stem(word);
};

export const stem = (language: Language, word: string): string =>
  getInstance(language).stem(word);
