# Multilingual stemmer

This package implements some stemmer algorithms found in the [snowball project](http://snowballstem.org/).

Under the hood is used `rust`, compiled by `webassembly` to work in `node.js`

# Supported Languages

-   Arabic
-   Danish
-   Dutch
-   English
-   French
-   German
-   Greek
-   Hungarian
-   Italian
-   Portuguese
-   Romanian
-   Russian
-   Spanish
-   Swedish
-   Tamil
-   Turkish

# Installation

```
npm i multilingual-stemmer
```

# Usage

```typescript
import { Stemmer, Languages } from 'multilingual-stemmer';

// Create a stemmer for the english language
const stemmer = new Stemmer(Languages.English);

// Stemm the word "fruitlessly"
// Please be aware that all languages expect their input to only contain lowercase characters.
console.log(stemmer.stem("fruitlessly")) // "fruitless" output

```