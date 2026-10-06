import * as native from '../index.js';

export const main = () => {
  return "Hello, world!";
};

export const Hash = native.Hash;
export const Hmac = native.Hmac;
export const TLS = native.Tls;
export const Tls = native.Tls;
export const TlsProvider = native.TlsProvider;

export const argon2 = native.argon2;
export const argon2Sync = native.argon2Sync;

export const createHash = native.createHash;
export const hash = native.hash;
export const getHashes = native.getHashes;

export const createHmac = native.createHmac;

export const pbkdf2 = native.pbkdf2;
export const pbkdf2Sync = native.pbkdf2Sync;

export default {
  main,
  Hash,
  Hmac,
  TLS,
  Tls,
  TlsProvider,
  argon2,
  argon2Sync,
  createHash,
  hash,
  getHashes,
  createHmac,
  pbkdf2,
  pbkdf2Sync,
};
