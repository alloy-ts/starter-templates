import { Ksuid, KsuidMs } from "../index.js";

export { Ksuid, KsuidMs };

export function generateKsuid(payload?: Uint8Array): string {
  return Ksuid.now(payload).toString();
}

export function generateKsuidMs(payload?: Uint8Array): string {
  return KsuidMs.now(payload).toString();
}

export const main = () => {
  const ksuid = Ksuid.now();
  return `KSUID: ${ksuid.toString()}`;
};
