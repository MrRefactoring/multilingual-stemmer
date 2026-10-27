import { describe, test, expect } from 'vitest';
import { Languages, createStemmer, stem, english, earlyModernEnglish, ukrainian } from '../dist/index.js';

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
    expect(() => createStemmer('xx' as 'en')).toThrow('Unknown language identifier');
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

describe.each([
  { name: 'Ukrainian', identifier: 'uk' as const, variant: 'Ukrainian', named: ukrainian, word: 'акварельний', expected: 'акварел' },
  { name: 'Early Modern English', identifier: 'earlymodernenglish' as const, variant: 'EarlyModernEnglish', named: earlyModernEnglish, word: 'bringeth', expected: 'bring' },
])('$name API', ({ identifier, variant, named, word, expected }) => {
  test('named export', () => {
    expect(named(word)).toBe(expected);
  });

  test('factory with string identifier', () => {
    expect(createStemmer(identifier)(word)).toBe(expected);
  });

  test('factory with enum', () => {
    expect(createStemmer(Languages[variant])(word)).toBe(expected);
  });

  test('stem with string identifier', () => {
    expect(stem(identifier, word)).toBe(expected);
  });

  test('stem with enum', () => {
    expect(stem(Languages[variant], word)).toBe(expected);
  });

  test('empty input', () => {
    expect(named('')).toBe('');
  });
});

describe('Snowball 3.2.0 regressions', () => {
  test.each([
    ["o'erflowing", 'overflow'],
    ['goſpel', 'gospel'],
    ['runneth', 'run'],
  ])('Early Modern English: %s', (word, expected) => {
    expect(earlyModernEnglish(word)).toBe(expected);
  });

  test('modern English handles historical forms independently', () => {
    expect(english('bringeth')).toBe('bringeth');
  });

  test('French colloquial elision', () => {
    expect(stem('fr', "z'amis")).toBe('amis');
  });
});
