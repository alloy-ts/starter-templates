# API Specifications & Compatibility Status

This document tabulates the standard `node:crypto` APIs alongside `@lib/crypto` implementation and compatibility status using NAPI-RS and Rust cryptographic libraries (`ring`, `rustls`, `argon2-rust`, `boring-rustls-provider`, `rustls-openssl`).

| Standard `node:crypto` API | `@lib/crypto` Export / Class | Status | Notes / Backend Provider |
| :--- | :--- | :--- | :--- |
| `crypto.createHash(algorithm[, options])` | `createHash(algorithm)` / `Hash` | **Supported** | Implemented in `src/crypto_hasher.rs` using `ring::digest`. Supports SHA-1, SHA-256, SHA-384, SHA-512, SHA-512/256. `update` and `digest` methods supported. |
| `crypto.hash(algorithm, data[, options])` | `hash(algorithm, data, encoding)` | **Supported** | One-shot hashing helper implemented in `src/crypto_hasher.rs`. |
| `crypto.getHashes()` | `getHashes()` | **Supported** | Returns array of supported hash digests (`sha1`, `sha256`, `sha384`, `sha512`, `sha512-256`). |
| `crypto.createHmac(algorithm, key[, options])` | `createHmac(algorithm, key)` / `Hmac` | **Supported** | Implemented in `src/hmac.rs` using `ring::hmac`. Supports `update` and `digest` (`hex`, `base64`, `buffer`). |
| `crypto.pbkdf2(password, salt, iterations, keylen, digest, callback)` | `pbkdf2(password, salt, iterations, keylen, digest)` | **Supported** | Implemented in `src/pbkdf2.rs` as async Promise-based function using `ring::pbkdf2`. |
| `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)` | `pbkdf2Sync(password, salt, iterations, keylen, digest)` | **Supported** | Synchronous PBKDF2 implementation in `src/pbkdf2.rs`. |
| `Argon2` (Extension API) | `argon2Hash`, `argon2HashSync`, `argon2VerifySync`, etc. | **Supported** | Implemented in `src/argon2.rs` using `argon2-rust`. Follows `@node-rs/argon2` specification. |
| `TLS` (Extension API) | `TLS` | **Supported** | Implemented in `src/tls.rs` using `rustls`. Supports `ring` (default) and `btls` (`boring-rustls-provider`). |
| `crypto.createCipheriv(...)` / `createDecipheriv(...)` | N/A | *Not Implemented* | Planned extension for symmetric encryption using Rust AEAD crates. |
| `crypto.randomBytes(size[, callback])` / `randomUUID()` | N/A | *Not Implemented* | Native JS `crypto.randomUUID()` / `webcrypto` can be used. |
| `crypto.generateKeyPair(...)` / `sign` / `verify` | N/A | *Not Implemented* | Asymmetric signing/verification planned for future release. |
| `crypto.hkdf(...)` / `hkdfSync(...)` | N/A | *Not Implemented* | Key derivation function extension planned. |
| `crypto.scrypt(...)` / `scryptSync(...)` | N/A | *Not Implemented* | Scrypt KDF planned. |
