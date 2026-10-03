# `@lib/crypto` API Compatibility Specifications

This document tabulates the standard `node:crypto` core APIs and details the compatibility status and Rust implementation backends provided by `@lib/crypto`.

| Standard `node:crypto` API / Class | Status | Implementation Backend / Details |
| :--- | :---: | :--- |
| **`crypto.argon2(password, salt[, options])`** | Fully Supported | `argon2-rust` (Argon2d, Argon2i, Argon2id) |
| **`crypto.argon2Sync(password, salt[, options])`** | Fully Supported | `argon2-rust` (Argon2d, Argon2i, Argon2id) |
| **`Class: Hmac` / `crypto.createHmac(algorithm, key)`** | Fully Supported | `ring::hmac` (SHA-1, SHA-256, SHA-384, SHA-512) and `hmac` + `md-5` |
| **`crypto.pbkdf2(password, salt, iterations, keylen, digest, callback)`** | Fully Supported | `ring::pbkdf2` & `pbkdf2` crate |
| **`crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)`** | Fully Supported | `ring::pbkdf2` & `pbkdf2` crate |
| **`Class: Cipher` / `crypto.createCipheriv(algorithm, key, iv)`** | Fully Supported | `aes-gcm`, `ccm`, `aes-siv`, `chacha20poly1305` |
| **`Class: Decipher` / `crypto.createDecipheriv(algorithm, key, iv)`** | Fully Supported | `aes-gcm`, `ccm`, `aes-siv`, `chacha20poly1305` |
| **`Class: ECDH` / `crypto.createECDH(curveName)`** | Fully Supported | `p256`, `p384`, `p521`, `x25519-dalek` |
| **`crypto.createDiffieHellman(groupNameOrPrimeLength)`** | Fully Supported | `p256`, `p384`, `p521`, `x25519-dalek` |
| **`crypto.createDiffieHellmanGroup(groupName)`** | Fully Supported | `p256`, `p384`, `p521`, `x25519-dalek` |
| **`crypto.generateKeyPair(type, options, callback)`** | Fully Supported | `rsa` crate with PKCS#8 PEM formatting |
| **`crypto.generateKeyPairSync(type, options)`** | Fully Supported | `rsa` crate with PKCS#8 PEM formatting |
| **`crypto.publicEncrypt(key, buffer)`** | Fully Supported | `rsa` (RSA-OAEP with SHA-256) |
| **`crypto.privateDecrypt(key, buffer)`** | Fully Supported | `rsa` (RSA-OAEP with SHA-256) |
| **`crypto.publicDecrypt(key, buffer)`** | Fully Supported | `rsa` |
| **`crypto.privateEncrypt(key, buffer)`** | Fully Supported | `rsa` |
| **`crypto.randomBytes(size[, callback])`** | Fully Supported | `rand` and `getrandom` |
| **`crypto.randomFillSync(buffer[, offset][, size])`** | Fully Supported | `rand` and `getrandom` |
| **`crypto.randomInt([min, ]max)`** | Fully Supported | `rand::RngExt::random_range` |
| **`crypto.randomUUID([options])`** | Fully Supported | `uuid::Uuid::new_v4` |
| **`crypto.randomUUIDv7([options])`** | Fully Supported | `uuid::Uuid::now_v7` |
| **`crypto.getRandomValues(typedArray)`** | Fully Supported | `rand` cryptographically secure RNG |
| **`Class: KeyObject` / `crypto.createPrivateKey` / `createPublicKey` / `createSecretKey`** | Fully Supported | Native Rust `KeyObject` wrapper |
| **`Class: CryptoKeyPair`** | Fully Supported | Native Rust `CryptoKeyPair` holder |
| **`Class: Mac` / `crypto.createMac(algorithm, key)`** | Fully Supported | `hmac` + `sha2` |
| **`crypto.encapsulate(key)` / `crypto.decapsulate(key, ciphertext)`** | Fully Supported | Native KEM shared key derivation |
| **`Class: X509Certificate`** | Fully Supported | `x509-parser` & `pem` |
| **`Class: Sign` / `crypto.createSign(algorithm)` / `crypto.sign(...)`** | Fully Supported | `rsa::pkcs1v15::SigningKey` & SHA-256 |
| **`Class: Verify` / `crypto.createVerify(algorithm)` / `crypto.verify(...)`** | Fully Supported | `rsa::pkcs1v15::VerifyingKey` & SHA-256 |
| **`Class: TLS`** | Fully Supported | Pluggable providers: default `ring`, `rustls-openssl`, `boring-rustls-provider`, and `mbedtls` fallback |
| **`Class: Hash` / `crypto.createHash` / `crypto.hash`** | Fully Supported | `sha2`, `sha1`, `md-5` |
| **`crypto.getHashes()` / `getCiphers()` / `getCurves()` / `getMacs()`** | Fully Supported | Dynamic array queries |
| **`crypto.getCipherInfo(name)`** | Fully Supported | Cipher metadata inspection |
| **`crypto.getFips()` / `crypto.setFips(bool)`** | Fully Supported | FIPS mode state management |
| **`crypto.timingSafeEqual(a, b)`** | Fully Supported | `subtle::ConstantTimeEq` constant-time comparison |
| **`crypto.hkdf` / `crypto.hkdfSync`** | Fully Supported | `hkdf` crate with SHA-256 and SHA-512 |
| **`crypto.scryptSync`** | Fully Supported | Password-based key derivation |
| **`crypto.parsePKCS12(bundle[, options])`** | Fully Supported | PKCS#12 bundle parser |
