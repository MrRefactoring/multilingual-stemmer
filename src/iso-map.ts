import type { IsoCode } from './interfaces/index.js';

export type IsoMap = Readonly<Record<IsoCode, number>>;

export const buildIsoMap = (langs: Record<string, number>): IsoMap => ({
  ar: langs['Arabic'],    hy: langs['Armenian'],  eu: langs['Basque'],
  ca: langs['Catalan'],   cs: langs['Czech'],      da: langs['Danish'],
  nl: langs['Dutch'],     en: langs['English'],    eo: langs['Esperanto'],
  et: langs['Estonian'],  fi: langs['Finnish'],    fr: langs['French'],
  de: langs['German'],    el: langs['Greek'],      hi: langs['Hindi'],
  hu: langs['Hungarian'], id: langs['Indonesian'], ga: langs['Irish'],
  it: langs['Italian'],   lt: langs['Lithuanian'], ne: langs['Nepali'],
  no: langs['Norwegian'], fa: langs['Persian'],    pl: langs['Polish'],
  pt: langs['Portuguese'],ro: langs['Romanian'],   ru: langs['Russian'],
  sr: langs['Serbian'],   es: langs['Spanish'],    sv: langs['Swedish'],
  ta: langs['Tamil'],     tr: langs['Turkish'],    yi: langs['Yiddish'],
}) as IsoMap;

export const buildResolver = (isoMap: IsoMap) =>
  (lang: number | IsoCode): number => {
    if (typeof lang === 'number') return lang;
    const resolved = isoMap[lang];
    if (resolved === undefined) throw new Error(`Unknown ISO language code: "${lang}"`);
    return resolved;
  };
