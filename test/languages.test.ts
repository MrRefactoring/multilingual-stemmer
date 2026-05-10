import { describe, test, expect } from 'vitest';
import { Languages, stem } from '../dist/index.js';
import { loadVoc, loadRes } from './utils/loadData.js';

const testLanguage = (langCode: string, language: number) => {
  test(String(language), () => {
    const voc = loadVoc(langCode);
    const res = loadRes(langCode);
    voc.forEach((word, i) => {
      expect(stem(language, word), `"${word}"`).toBe(res[i]);
    });
  });
};

describe('Stemming algorithms', () => {
  testLanguage('ar',  Languages.Arabic);
  testLanguage('hy',  Languages.Armenian);
  testLanguage('eu',  Languages.Basque);
  testLanguage('ca',  Languages.Catalan);
  testLanguage('cs',  Languages.Czech);
  testLanguage('da',  Languages.Danish);
  testLanguage('nl',  Languages.Dutch);
  testLanguage('en',  Languages.English);
  testLanguage('eo',  Languages.Esperanto);
  testLanguage('et',  Languages.Estonian);
  testLanguage('fi',  Languages.Finnish);
  testLanguage('fr',  Languages.French);
  testLanguage('de',  Languages.German);
  testLanguage('el',  Languages.Greek);
  testLanguage('hi',  Languages.Hindi);
  testLanguage('hu',  Languages.Hungarian);
  testLanguage('id',  Languages.Indonesian);
  testLanguage('ga',  Languages.Irish);
  testLanguage('it',  Languages.Italian);
  testLanguage('lt',  Languages.Lithuanian);
  testLanguage('ne',  Languages.Nepali);
  testLanguage('no',  Languages.Norwegian);
  testLanguage('fa',  Languages.Persian);
  testLanguage('pl',  Languages.Polish);
  testLanguage('pt',  Languages.Portuguese);
  testLanguage('ro',  Languages.Romanian);
  testLanguage('ru',  Languages.Russian);
  testLanguage('sr',  Languages.Serbian);
  testLanguage('es',  Languages.Spanish);
  testLanguage('sv',  Languages.Swedish);
  testLanguage('ta',  Languages.Tamil);
  testLanguage('tr',  Languages.Turkish);
  testLanguage('yi',  Languages.Yiddish);
});
