import * as native from '../index.js';

export const main = () => {
  return "Hello, world!";
};

export const Hash = native.Hash;
export const Hmac = native.Hmac;
export const TLS = native.Tls;
export const Tls = native.Tls;
export const TlsProvider = native.TlsProvider;

export const Algorithm = native.Algorithm;
export const Version = native.Version;

export const argon2Hash = native.argon2Hash;
export const argon2HashSync = native.argon2HashSync;
export const argon2HashRaw = native.argon2HashRaw;
export const argon2HashRawSync = native.argon2HashRawSync;
export const argon2Verify = native.argon2Verify;
export const argon2VerifySync = native.argon2VerifySync;
export const argon2ParseOptions = native.argon2ParseOptions;
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
  Algorithm,
  Version,
  argon2Hash,
  argon2HashSync,
  argon2HashRaw,
  argon2HashRawSync,
  argon2Verify,
  argon2VerifySync,
  argon2ParseOptions,
  argon2,
  argon2Sync,
  createHash,
  hash,
  getHashes,
  createHmac,
  pbkdf2,
  pbkdf2Sync,
};
