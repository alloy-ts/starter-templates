import type { KsuidOptions } from "../index.d.ts";
import { Ksuid, KsuidMs } from "../index.js";

export { Ksuid, KsuidMs, type KsuidOptions };

export function generateKsuid(payload?: Uint8Array, options?: string | KsuidOptions): string {
  if (options && typeof options === "object" && options.timestampSize) {
    const k = new Ksuid(undefined, options);
    return k.toString(options);
  }
  return Ksuid.now(payload).toString(options);
}

export function generateKsuidMs(payload?: Uint8Array, options?: string | KsuidOptions): string {
  return KsuidMs.now(payload).toString(options);
}

export const main = () => {
  const ksuid = Ksuid.now();
  return `KSUID: ${ksuid.toString()}`;
};
