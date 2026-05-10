import { describe, test, expect } from 'vitest';
import { Languages, createStemmer, stem, english } from '../dist/index.js';

describe('createStemmer', () => {
  test('returns a function', () => {
    expect(typeof createStemmer(Languages.English)).toBe('function');
  });

  test('stems correctly', () => {
    const stemWord = createStemmer(Languages.English);
    expect(stemWord('fruitlessly')).toBe('fruitless');
  });

  test('is idempotent', () => {
    const stemWord = createStemmer(Languages.English);
    const once = stemWord('running');
    expect(stemWord(once)).toBe(once);
  });

  test('handles empty string', () => {
    const stemWord = createStemmer(Languages.English);
    expect(stemWord('')).toBe('');
  });

  test('accepts ISO code', () => {
    const stemWord = createStemmer('en');
    expect(stemWord('fruitlessly')).toBe('fruitless');
  });

  test('throws on unknown ISO code', () => {
    expect(() => createStemmer('xx' as 'en')).toThrow('Unknown ISO language code');
  });
});

describe('stem', () => {
  test('stems correctly with enum', () => {
    expect(stem(Languages.English, 'fruitlessly')).toBe('fruitless');
  });

  test('stems correctly with ISO code', () => {
    expect(stem('en', 'fruitlessly')).toBe('fruitless');
  });

  test('handles empty string', () => {
    expect(stem(Languages.English, '')).toBe('');
  });
});

describe('language functions', () => {
  test('english()', () => {
    expect(english('fruitlessly')).toBe('fruitless');
  });
});
