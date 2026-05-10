import type { WasmBinding } from '../interfaces/index.js';

export declare const Stemmer: WasmBinding['Stemmer'];
export declare const Languages: WasmBinding['Languages'];
export default function init(input?: URL | BufferSource | string): Promise<void>;
