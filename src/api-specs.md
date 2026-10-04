# `@lib/crypto` API Specifications & Node.js Compatibility Matrix

This document tabulates the standard `node:crypto` API, our native NAPI-RS implementations, and their compatibility status.

## Standard `node:crypto` API Specifications & Compatibility Status

| Category / Class | API / Method | `node:crypto` Contract / Specification | `@lib/crypto` Implementation | Status | Notes |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **`crypto.Hash`** | `crypto.createHash(algorithm[, options])` | Creates `Hash` instance for computing message digests. | Implemented in `src/crypto_hasher.rs` using `ring` / `rustls`. | **Supported** | Supports `sha1`, `sha256`, `sha384`, `sha512`, `sha512-256`. |
| | `hash.update(data[, inputEncoding])` | Updates hash content with data. | Implemented on `Hash` class in `src/crypto_hasher.rs`. | **Supported** | Accepts string or Buffer / Uint8Array. |
| | `hash.digest([encoding])` | Calculates digest value. | Implemented on `Hash` class in `src/crypto_hasher.rs`. | **Supported** | Supports `hex`, `base64`, `latin1`, `utf8`, `buffer` output encodings. |
| | `crypto.hash(algorithm, data[, outputEncoding])` | One-shot digest calculation. | Implemented in `src/crypto_hasher.rs`. | **Supported** | Fast one-shot hashing utility for supported digest algorithms. |
| | `crypto.getHashes()` | Returns array of supported hash algorithm names. | Implemented in `src/crypto_hasher.rs`. | **Supported** | Returns available digest algorithm identifiers (`['sha1', 'sha256', 'sha384', 'sha512', 'sha512-256']`). |
| **`crypto.Hmac`** | `crypto.createHmac(algorithm, key[, options])` | Creates `Hmac` instance using secret key. | Implemented in `src/hmac.rs` using `ring::hmac`. | **Supported** | Supports `sha1`, `sha256`, `sha384`, `sha512` HMAC. |
| | `hmac.update(data[, inputEncoding])` | Updates HMAC content with data. | Implemented on `Hmac` class in `src/hmac.rs`. | **Supported** | Accepts string or Buffer / Uint8Array. |
| | `hmac.digest([encoding])` | Calculates HMAC digest. | Implemented on `Hmac` class in `src/hmac.rs`. | **Supported** | Supports `hex`, `base64`, `latin1`, `utf8`, `buffer` encodings. |
| **Key Derivation** | `crypto.pbkdf2(password, salt, iterations, keylen, digest, callback)` | Async PBKDF2 key derivation. | Implemented in `src/pbkdf2.rs` using `ring::pbkdf2`. | **Supported** | Non-blocking thread pool execution via NAPI `AsyncTask`. |
| | `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)` | Sync PBKDF2 key derivation. | Implemented in `src/pbkdf2.rs` using `ring::pbkdf2`. | **Supported** | Returns derived key as Buffer. |
| **Argon2** | `crypto.argon2(algorithm, parameters, callback)` | Async Argon2 key derivation (`argon2d`, `argon2i`, `argon2id`). | Implemented in `src/argon2.rs` using `argon2-rust`. | **Supported** | Accepts `algorithm` string and `Argon2Parameters` (`message`, `nonce`, `parallelism`, `tagLength`, `memory`, `passes`, `secret`, `associatedData`). |
| | `crypto.argon2Sync(algorithm, parameters)` | Sync Argon2 key derivation. | Implemented in `src/argon2.rs` using `argon2-rust`. | **Supported** | Synchronous Argon2 derivation matching `node:crypto` signature. |
| | `argon2Hash` / `argon2Verify` / `argon2ParseOptions` | Extended helper utilities for Argon2 PHC string hashing and verification. | Implemented in `src/argon2.rs`. | **Supported Extension** | Supports PHC string parsing & verification. |
| **TLS** | `TLS` Class | Multi-backend TLS provider selection. | Implemented in `src/tls.rs`. | **Supported Extension** | Configurable providers: `Ring` (default), `OpenSSL`, `Btls` (BoringSSL). |

## `no-std` and POSIX Compatibility

The crate relies on `alloc` and `core` abstractions for data structures, allowing underlying cryptographic core functions to operate in `no-std` target environments. OS entropy is sourced via system calls (`getrandom`/`getentropy`) or NAPI host environment bindings.
