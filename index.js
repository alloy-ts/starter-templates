import { createRequire } from "node:module";
import path from "node:path";
import process from "node:process";

const require = createRequire(import.meta.url);

function loadBinding() {
  const possiblePaths = [
    path.resolve(process.cwd(), "lib-crypto.node"),
    path.resolve(process.cwd(), "dist/artifacts/lib-crypto.node"),
    path.resolve(process.cwd(), "dist/artifacts/my-addon.node"),
  ];
  for (const p of possiblePaths) {
    try {
      return require(p);
    } catch {}
  }
  throw new Error("Failed to load native binding from possible paths: " + possiblePaths.join(", "));
}

const nativeBinding = loadBinding();

export const {
  CryptoKeyPair,
  ECDH,
  Ecdh,
  Hash,
  Hmac,
  KeyObject,
  Sign,
  TLS,
  Tls,
  Verify,
  X509Certificate,
  argon2,
  argon2Sync,
  createDiffieHellman,
  createDiffieHellmanGroup,
  createECDH,
  createEcdh,
  createHash,
  createHmac,
  createMac,
  createPrivateKey,
  createPublicKey,
  createSecretKey,
  createSign,
  createTLS,
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
  randomUUID,
  randomUuid,
  randomUUIDv7,
  randomUuidv7,
  scrypt,
  scryptSync,
  sign,
  verify,
} = nativeBinding;

export default nativeBinding;
