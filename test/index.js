const fs = require('fs');
const path = require('path');
const assert = require('assert');

const { Stemmer, Languages } = require('../dist');

const dataPath = path.join(__dirname, 'data');

const loadData = (filename) => {
    return fs
        .readFileSync(path.join(dataPath, filename))
        .toString()
        .split('\n')
        .map(word => word.trim());
};

const testLanguage = (language) => {
    const langCode = language.slice();

    switch(language) {
        case 'ar':
            language = Languages.Arabic;
            break;
        case 'el':
            language = Languages.Greek;
            break;
        case 'en':
            language = Languages.English;
            break;
        case 'es':
            language = Languages.Spanish;
            break;
        case 'fi':
            language = Languages.Finnish;
            break;
        case 'fr':
            language = Languages.French;
            break;
        case 'ger':
            language = Languages.German;
            break;
        case 'it':
            language = Languages.Italian;
            break;
        case 'pt':
            language = Languages.Portuguese;
            break;
        case 'ro':
            language = Languages.Romanian;
            break;
        case 'ru':
            language = Languages.Russian;
            break;
    }

    const stemmer = new Stemmer(language);

    const res = loadData(`res_${langCode}.txt`);
    const voc = loadData(`voc_${langCode}.txt`);

    voc.forEach((word, index) => {
        assert(stemmer.stem(word) === res[index]);
    });
};

testLanguage('ar');
testLanguage('el');
testLanguage('en');
testLanguage('es');
testLanguage('fi');
testLanguage('fr');
testLanguage('ger');
testLanguage('it');
testLanguage('pt');
testLanguage('ro');
testLanguage('ru');
