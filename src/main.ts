import binding from "../index.js";

export const {
  CryptoKeyPair,
  Ecdh,
  Hash,
  Hmac,
  KeyObject,
  Sign,
  Tls,
  Verify,
  X509Certificate,
  argon2,
  argon2Sync,
  createDiffieHellman,
  createDiffieHellmanGroup,
  createEcdh,
  createHash,
  createHmac,
  createMac,
  createPrivateKey,
  createPublicKey,
  createSecretKey,
  createSign,
  createTls,
  createVerify,
  decapsulate,
  decryptAead,
  encapsulate,
  encryptAead,
  generateKeyPairSync,
  getHashes,
  hash,
  hkdf,
  hkdfSync,
  parsePkcs8,
  pbkdf2,
  pbkdf2Sync,
  privateDecrypt,
  privateEncrypt,
  publicDecrypt,
  publicEncrypt,
  randomBytes,
  randomBytesSync,
  randomFill,
  randomFillSync,
  randomInt,
  randomUuid,
  randomUuidv7,
  scrypt,
  scryptSync,
  sign,
  verify,
} = binding;

export const ECDH = binding.Ecdh;
export const createECDH = binding.createEcdh;
export const TLS = binding.Tls;
export const createTLS = binding.createTls;
export const randomUUID = binding.randomUuid;
export const randomUUIDv7 = binding.randomUuidv7;

export default binding;
