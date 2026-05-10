import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import init, { Stemmer, Languages } from './browser/index.js';

await init(readFileSync(fileURLToPath(new URL('./browser/index_bg.wasm', import.meta.url))));

export { Stemmer, Languages };
