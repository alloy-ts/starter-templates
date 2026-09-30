import type { EncodingOptions } from "../index.d.ts";
import { Ksuid, KsuidMs } from "../index.js";

export { Ksuid, KsuidMs, type EncodingOptions };

export function generateKsuid(payload?: Uint8Array, options?: string | EncodingOptions): string {
  return Ksuid.now(payload).toString(options);
}

export function generateKsuidMs(payload?: Uint8Array, options?: string | EncodingOptions): string {
  return KsuidMs.now(payload).toString(options);
}

export const main = () => {
  const ksuid = Ksuid.now();
  return `KSUID: ${ksuid.toString()}`;
};
