import { unlinkSync } from 'node:fs';
import { join } from 'node:path';

const browser = join(import.meta.dirname, 'dist', 'browser');

for (const f of ['package.json', 'LICENSE', 'README.md', '.gitignore']) {
  unlinkSync(join(browser, f));
}
