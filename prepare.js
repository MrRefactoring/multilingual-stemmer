const fs = require('fs');
const path = require('path');

const distFolder = path.join(__dirname, 'dist');

const package = path.join(distFolder, 'package.json');
const license = path.join(distFolder, 'LICENSE');
const readme = path.join(distFolder, 'README.md');
const gitignore = path.join(distFolder, '.gitignore');

[package, license, readme, gitignore].forEach(f => fs.unlinkSync(f));
