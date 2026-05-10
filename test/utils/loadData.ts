import { readFileSync } from 'node:fs';
import { join } from 'node:path';

const dataPath = join(import.meta.dirname, '../data');

export const loadVoc = (langCode: string): string[] =>
  readFileSync(join(dataPath, `voc_${langCode}.txt`), 'utf8')
    .split('\n')
    .map(word => word.trim());

export const loadRes = (langCode: string): string[] =>
  readFileSync(join(dataPath, `res_${langCode}.txt`), 'utf8')
    .split('\n')
    .map(word => word.trim());
