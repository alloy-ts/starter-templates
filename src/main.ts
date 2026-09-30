import type { KsuidOptions as NativeKsuidOptions } from "../index.d.ts";
import { Ksuid, KsuidMs } from "../index.js";

export type KsuidOptions = {
  enc?: "base62" | "crockford" | "base32" | (string & {});
  alphabet?: string;
  timestampSize?:
    | "32bit"
    | "48bit"
    | "64bit"
    | "4bytes"
    | "6bytes"
    | "8bytes"
    | 32
    | 48
    | 64
    | 4
    | 6
    | 8
    | (string & {})
    | (number & {});
};

export { Ksuid, KsuidMs };

export function generateKsuid(payload?: Uint8Array, options?: string | KsuidOptions): string {
  if (options && typeof options === "object" && options.timestampSize) {
    const k = new Ksuid(undefined, options as NativeKsuidOptions);
    return k.toString(options as NativeKsuidOptions);
  }
  return Ksuid.now(payload).toString(options as NativeKsuidOptions);
}

export function generateKsuidMs(payload?: Uint8Array, options?: string | KsuidOptions): string {
  return KsuidMs.now(payload).toString(options as NativeKsuidOptions);
}

export const main = () => {
  const ksuid = Ksuid.now();
  return `KSUID: ${ksuid.toString()}`;
};
