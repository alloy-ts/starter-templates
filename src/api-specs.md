# Node:Crypto Compatibility Matrix & Specifications

This document tabulates the standard Node.js `node:crypto` core API and the implementation / compatibility status provided by the native `@lib/crypto` Rust NAPI module (`lib_crypto_native`).

## Default Providers

- Primary default cryptographic provider in Rust: `ring` and `rustls`
- Native Rust cryptographic equivalents (`argon2`, `aes-gcm`, `rsa`, `pbkdf2`, `uuid`, `pkcs8`, `x509-parser`)

---

## API Compatibility Table

| Node.js `node:crypto` API                     | Status  | Provider / Implementation     | Notes                                                 |
| :-------------------------------------------- | :------ | :---------------------------- | :---------------------------------------------------- |
| `crypto.createHash(algorithm)`                | ✅ Full | `ring::digest`                | Supports `sha1`, `sha256`, `sha384`, `sha512`         |
| `crypto.getHashes()`                          | ✅ Full | Native Rust (`crypto_hasher`) | Returns list of supported digest algorithms           |
| `crypto.createHmac(algorithm, key)`           | ✅ Full | `ring::hmac`                  | `Hmac` class with `update` and `digest`               |
| `crypto.createMac(algorithm, key[, options])` | ✅ Full | `ring::hmac`                  | MAC factory supporting HMAC algorithms                |
| `crypto.pbkdf2(password, salt, ...)`          | ✅ Full | `ring::pbkdf2`                | Async and sync (`pbkdf2Sync`) support                 |
| `crypto.argon2(password, salt, ...)`          | ✅ Full | `argon2` crate                | Async and sync (`argon2Sync`) support                 |
| `crypto.hkdf(digest, ikm, salt, ...)`         | ✅ Full | `ring::hkdf`                  | Async and sync (`hkdfSync`) support                   |
| `crypto.cipherEncrypt` / `cipherDecrypt`      | ✅ Full | `aes-gcm`                     | AES-128-GCM and AES-256-GCM AEAD ciphers              |
| `crypto.createECDH(curveName)`                | ✅ Full | `ring::agreement` (X25519)    | `Ecdh` class with `generateKeys` and `computeSecret`  |
| `crypto.getCurves()`                          | ✅ Full | Native Rust (`ecdh`)          | Returns list of supported elliptic curves             |
| `crypto.generateKeyPair` / `Sync`             | ✅ Full | `rsa` & `pkcs8`               | RSA key pair generation with PKCS#8 PEM output        |
| `crypto.randomBytes(size)`                    | ✅ Full | `ring::rand::SystemRandom`    | Cryptographically secure random byte buffer           |
| `crypto.randomFillSync(buffer)`               | ✅ Full | `ring::rand::SystemRandom`    | In-place random buffer fill                           |
| `crypto.randomInt(min, max)`                  | ✅ Full | `ring::rand::SystemRandom`    | Bias-free random integer generation                   |
| `crypto.randomUUID()`                         | ✅ Full | `uuid::Uuid::new_v4`          | RFC 4122 v4 UUID string                               |
| `crypto.randomUUIDv7()`                       | ✅ Full | `uuid::Uuid::now_v7`          | RFC 9562 v7 time-ordered UUID string                  |
| `crypto.KeyObject` / `createSecretKey`        | ✅ Full | Native Rust (`key_object`)    | KeyObject representation and export                   |
| `crypto.createPublicKey` / `PrivateKey`       | ✅ Full | Native Rust (`agreement`)     | KeyObject wrapper constructors                        |
| `crypto.diffieHellman(privKey, pubKey)`       | ✅ Full | `ring::agreement`             | Computes shared secret                                |
| `crypto.encapsulate` / `decapsulate`          | ✅ Full | Native Rust (`agreement`)     | Key encapsulation mechanism (KEM) APIs                |
| `crypto.createSign` / `Sign` class            | ✅ Full | `ring::signature` (Ed25519)   | Stream signature generation                           |
| `crypto.createVerify` / `Verify` class        | ✅ Full | `ring::signature` (Ed25519)   | Signature verification                                |
| `crypto.X509Certificate`                      | ✅ Full | `x509-parser`                 | X.509 certificate parser (`subject`, `issuer`, `raw`) |
| `TLS` class                                   | ✅ Full | Native NAPI (`tls`)           | Default provider using `ring`                         |
| `parsePkcs8Pem` / `exportPkcs8Pem`            | ✅ Full | `pkcs8`                       | PKCS#8 PEM parsing and DER encoding                   |
