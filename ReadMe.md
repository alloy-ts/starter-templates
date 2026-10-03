# `@lib/crypto` (`lib_crypto_native`)

Node.js `node:crypto` compatible native cryptographic addon powered by Rust and [NAPI-RS](https://napi.rs/).

## Features

- **Hash & Hasher**: `crypto.createHash`, `crypto.hash`, `crypto.getHashes`, `Hash` class using `ring` / `rustls`.
- **HMAC**: `crypto.createHmac`, `Hmac` class supporting `sha1`, `sha256`, `sha384`, `sha512` with multiple output encodings (`hex`, `base64`, `latin1`, `utf8`, `buffer`).
- **PBKDF2**: Async and sync key derivation via `crypto.pbkdf2` and `crypto.pbkdf2Sync`.
- **Argon2**: PHC string hashing and verification (`argon2Hash`, `argon2Verify`, `argon2ParseOptions`, `crypto.argon2`, `crypto.argon2Sync`).
- **TLS Multi-Backend**: Flexible provider selection for `TLS` (`Ring`, `OpenSSL`, `BTLS`, `MbedTLS`) with default `Ring` fallback.

## Supporting `no-std` & POSIX Targets

To support `no-std` or minimal POSIX environments without requiring `std` (`libstd`):

1. **Crate Level `#![no_std]`**: Use `#![no_std]` at the top of crate modules and bring in dynamic memory allocation via `extern crate alloc;`.
2. **Core Types**: Rely on `core::*` types (`core::slice`, `core::fmt`, `core::str`, `core::result::Result`) instead of `std::*`.
3. **Core Crypto Crates**: Standardize on pure Rust crypto crates (`ring`, `argon2-rust`, `rustls`) that support `default-features = false` with `alloc` enabled.
4. **POSIX System Fallbacks**: On POSIX targets without `std` OS bindings, platform entropy can be retrieved directly via `getrandom` with syscall fallback (`getrandom`, `getentropy`, `/dev/urandom`) or OS libuv bindings provided by NAPI.

## Installation & Usage

```js
import { createHash, createHmac, pbkdf2Sync, argon2Sync, TLS, TlsProvider } from '@lib/crypto';

// Hashing
const digest = createHash('sha256').update('hello world').digest('hex');

// HMAC
const hmac = createHmac('sha256', 'secret-key').update('data').digest('hex');

// PBKDF2
const derivedKey = pbkdf2Sync('password', 'salt', 1000, 32, 'sha256');

// Argon2
const phcHash = argon2Sync('password', { timeCost: 2, memoryCost: 19456 });

// TLS Provider Selection
const tls = new TLS(TlsProvider.Ring);
console.log(tls.getCipherSuites());
```
