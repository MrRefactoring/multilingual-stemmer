import type { Language } from './interfaces/index.js';

export type LanguageMap = Readonly<Record<Exclude<Language, number>, number>>;

export const buildLanguageMap = (langs: Record<string, number>): LanguageMap => ({
  ar: langs['Arabic'],    hy: langs['Armenian'],  eu: langs['Basque'],
  ca: langs['Catalan'],   cs: langs['Czech'],      da: langs['Danish'],
  nl: langs['Dutch'],     en: langs['English'],    eo: langs['Esperanto'],
  et: langs['Estonian'],  fi: langs['Finnish'],    fr: langs['French'],
  de: langs['German'],    el: langs['Greek'],      hi: langs['Hindi'],
  hu: langs['Hungarian'], id: langs['Indonesian'], ga: langs['Irish'],
  it: langs['Italian'],   lt: langs['Lithuanian'], ne: langs['Nepali'],
  no: langs['Norwegian'], fa: langs['Persian'],    pl: langs['Polish'],
  pt: langs['Portuguese'],ro: langs['Romanian'],   ru: langs['Russian'],
  sr: langs['Serbian'],   st: langs['Sesotho'],    es: langs['Spanish'],
  sv: langs['Swedish'],   ta: langs['Tamil'],      tr: langs['Turkish'],
  uk: langs['Ukrainian'],  yi: langs['Yiddish'],
  earlymodernenglish: langs['EarlyModernEnglish'],
});

export const buildResolver = (languageMap: LanguageMap) =>
  (lang: Language): number => {
    if (typeof lang === 'number') return lang;
    const resolved = languageMap[lang];
    if (resolved === undefined) throw new Error(`Unknown language identifier: "${lang}"`);
    return resolved;
  };
