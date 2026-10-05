# `@lib/crypto` API Specification & Compatibility Status

This document tabulates the standard `node:crypto` APIs and the implementation/compatibility status provided by `@lib/crypto` powered by NAPI-RS and native Rust cryptographic crates (`ring`, `rustls`, `argon2-rust`, `aes-gcm`, etc.).

| Category | API | Status | Notes |
| --- | --- | --- | --- |
| **Argon2** | `crypto.argon2(algorithm, parameters, callback)` | Supported | Asynchronous Argon2d, Argon2i, Argon2id KDF |
| | `crypto.argon2Sync(algorithm, parameters)` | Supported | Synchronous Argon2 KDF |
| **HMAC & MAC** | `crypto.createHmac(algorithm, key, options)` | Supported | HMAC class backed by `ring` |
| | `crypto.createMac(algorithm, key, options)` | Supported | Mac class backed by Rust crypto crates |
| | `Class: Hmac` | Supported | `update(data)`, `digest(encoding)` |
| | `Class: Mac` | Supported | `update(data)`, `final(outputEncoding)` |
| **Hash Functions** | `crypto.createHash(algorithm, options)` | Supported | Hash class backed by `sha2`, `sha1`, `md5` |
| | `crypto.hash(algorithm, data, options)` | Supported | One-shot hashing utility |
| | `Class: Hash` | Supported | `update(data)`, `digest(encoding)` |
| **AEAD & Ciphers** | `crypto.createCipheriv(algorithm, key, iv, options)` | Supported | Supports AES-GCM, ChaCha20-Poly1305, etc. |
| | `crypto.createDecipheriv(algorithm, key, iv, options)` | Supported | Supports AEAD decryption & tag verification |
| | `Class: Cipheriv` | Supported | `setAAD()`, `update()`, `final()`, `getAuthTag()` |
| | `Class: Decipheriv` | Supported | `setAAD()`, `setAuthTag()`, `update()`, `final()` |
| **Key Agreement** | `crypto.createECDH(curveName)` | Supported | ECDH class with X25519 support |
| | `Class: ECDH` | Supported | `generateKeys()`, `computeSecret()`, `getPrivateKey()`, `getPublicKey()`, `setPrivateKey()`, `setPublicKey()` |
| | `crypto.createDiffieHellman(prime, generator)` | Supported | DiffieHellman class |
| | `crypto.createDiffieHellmanGroup(name)` | Supported | DiffieHellmanGroup class |
| **Keys & PKI** | `Class: KeyObject` | Supported | `type`, `asymmetricKeyType`, `symmetricKeySize`, `export()`, `equals()` |
| | `crypto.createSecretKey(key, encoding)` | Supported | Symmetric secret KeyObject |
| | `crypto.createPublicKey(key)` | Supported | Public KeyObject |
| | `crypto.createPrivateKey(key)` | Supported | Private KeyObject |
| | `Class: Sign` | Supported | `update()`, `sign()` |
| | `Class: Verify` | Supported | `update()`, `verify()` |
| | `crypto.sign(algorithm, data, key)` | Supported | One-shot signature generation |
| | `crypto.verify(algorithm, data, key, signature)` | Supported | One-shot signature verification |
| | `Class: X509Certificate` | Supported | `subject`, `issuer`, `fingerprint`, `fingerprint256`, `fingerprint512`, `checkEmail()`, `checkHost()`, `checkIP()`, `verify()` |
| | `Class: Certificate` | Supported | `exportChallenge()`, `exportPublicKey()`, `verifySpkac()` |
| | `crypto.parsePKCS12(bundle, options)` | Supported | PKCS#12 bundle parser |
| | `crypto.encapsulate(key)` | Supported | KEM encapsulation |
| | `crypto.decapsulate(key, ciphertext)` | Supported | KEM decapsulation |
| | `crypto.generateKeyPairSync(type, options)` | Supported | Asymmetric key pair generation |
| | `crypto.generateKeySync(type, options)` | Supported | Symmetric secret key generation |
| **PBKDF2 / HKDF / Scrypt** | `crypto.pbkdf2(password, salt, iterations, keylen, digest)` | Supported | Asynchronous PBKDF2 |
| | `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)` | Supported | Synchronous PBKDF2 |
| | `crypto.hkdf(digest, ikm, salt, info, keylen)` | Supported | Asynchronous HKDF |
| | `crypto.hkdfSync(digest, ikm, salt, info, keylen)` | Supported | Synchronous HKDF |
| | `crypto.scrypt(password, salt, keylen, options)` | Supported | Asynchronous Scrypt |
| | `crypto.scryptSync(password, salt, keylen, options)` | Supported | Synchronous Scrypt |
| **TLS & Providers** | `Class: TLS` | Supported | Supports `Ring`, `OpenSSL`, `BoringSSL`, `MbedTLS` |
| **Utilities & Constants** | `crypto.randomBytes(size)` | Supported | CS-PRNG random bytes |
| | `crypto.randomFillSync(buffer, offset, size)` | Supported | CS-PRNG random fill |
| | `crypto.randomInt(min, max)` | Supported | Uniform random integer |
| | `crypto.randomUUID()` | Supported | RFC 4122 v4 UUID |
| | `crypto.randomUUIDv7()` | Supported | RFC 9562 v7 UUID |
| | `crypto.generatePrimeSync(size, options)` | Supported | Prime number generation |
| | `crypto.checkPrimeSync(candidate, options)` | Supported | Primality checking |
| | `crypto.timingSafeEqual(a, b)` | Supported | Constant-time buffer comparison |
| | `crypto.getCiphers()` | Supported | List of available ciphers |
| | `crypto.getHashes()` | Supported | List of available hash algorithms |
| | `crypto.getCurves()` | Supported | List of supported elliptic curves |
| | `crypto.getMacs()` | Supported | List of supported MAC algorithms |
| | `crypto.getCipherInfo(name)` | Supported | Cipher metadata lookup |
| | `crypto.getFips()`, `crypto.setFips()` | Supported | FIPS mode query/toggle |
| | `crypto.setEngine(engine, flags)` | Supported | Custom engine stub |
| | `crypto.secureHeapUsed()` | Supported | Secure heap statistics |
