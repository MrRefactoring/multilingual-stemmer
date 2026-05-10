export interface StemmerInstance {
  stem(word: string): string;
}

export interface WasmBinding {
  readonly Stemmer: new (language: number) => StemmerInstance;
  readonly Languages: Record<string, number>;
}

export type IsoCode =
  | 'ar' | 'hy' | 'eu' | 'ca' | 'cs' | 'da' | 'nl' | 'en' | 'eo' | 'et'
  | 'fi' | 'fr' | 'de' | 'el' | 'hi' | 'hu' | 'id' | 'ga' | 'it' | 'lt'
  | 'ne' | 'no' | 'fa' | 'pl' | 'pt' | 'ro' | 'ru' | 'sr' | 'es' | 'sv'
  | 'ta' | 'tr' | 'yi';

export type Language = number | IsoCode;
