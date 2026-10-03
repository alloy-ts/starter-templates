# `@lib/crypto` API Specifications & Node.js Compatibility Matrix

This document tabulates the standard `node:crypto` API, our native NAPI-RS implementations, and their compatibility status.

| API / Module | `node:crypto` Contract / Specification | `@lib/crypto` Implementation | Status | Notes |
| :--- | :--- | :--- | :--- | :--- |
| **`crypto.createHash` / `crypto.Hash`** | `crypto.createHash(algorithm[, options])` creates `Hash` instance for computing message digests. | Implemented in `src/crypto_hasher.rs` using `ring` / `rustls` digests. | **Supported** | Supports `sha1`, `sha256`, `sha384`, `sha512`, `sha512-256`. Supports `hex`, `base64`, `latin1`, `utf8`, `buffer` output encodings. |
| **`crypto.hash`** | `crypto.hash(algorithm, data[, outputEncoding])` one-shot digest calculation. | Implemented in `src/crypto_hasher.rs`. | **Supported** | Fast one-shot hashing utility for supported digest algorithms. |
| **`crypto.getHashes`** | Returns array of supported hash algorithm names. | Implemented in `src/crypto_hasher.rs`. | **Supported** | Returns available digest algorithm identifiers (`['sha1', 'sha256', 'sha384', 'sha512', 'sha512-256']`). |
| **`crypto.createHmac` / `crypto.Hmac`** | `crypto.createHmac(algorithm, key[, options])` creates `Hmac` instance using secret key. | Implemented in `src/hmac.rs` using `ring::hmac`. | **Supported** | Supports `sha1`, `sha256`, `sha384`, `sha512` HMAC. Supports `hex`, `base64`, `latin1`, `utf8`, `buffer` encodings. |
| **`crypto.pbkdf2` / `crypto.pbkdf2Sync`** | Async and sync Password-Based Key Derivation Function 2 (PBKDF2). | Implemented in `src/pbkdf2.rs` using `ring::pbkdf2`. | **Supported** | Supports `sha1`, `sha256`, `sha384`, `sha512` digest algorithms. Non-blocking thread pool execution for async `pbkdf2`. |
| **Argon2 (`crypto.argon2`, `crypto.argon2Sync`, etc.)** | Modern password hashing standard (PHC winner format). Extended crypto API. | Implemented in `src/argon2.rs` using `argon2-rust`. | **Supported** | Supports `argon2d`, `argon2i`, `argon2id` algorithms, PHC string formatting/parsing, custom time/memory cost & parallelism. Exposes `argon2` and `argon2Sync`. |
| **`TLS` Class / Multi-backend TLS** | Configurable TLS provider selection for `rustls`. | Implemented in `src/tls.rs` using `rustls-openssl`, `boring-rustls-provider`, `rustls-mbedcrypto-provider`, and `ring`. | **Supported** | Allows selecting underlying TLS provider (`Ring`, `OpenSSL`, `Btls`, `MbedTLS`) with default `Ring` fallback. |

## `no-std` and POSIX Compatibility

The crate relies on `alloc` and `core` abstractions for data structures, allowing underlying cryptographic core functions to operate in `no-std` target environments. OS entropy is sourced via system calls (`getrandom`/`getentropy`) or NAPI host environment bindings.
