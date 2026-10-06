# `@lib/crypto` Native Node:crypto API Specifications & Compatibility Matrix

`@lib/crypto` (Cargo package `lib_crypto_native`) is a Node.js native addon written in Rust using **NAPI-RS**. It provides a `node:crypto` compatible implementation backed by primary Rust cryptographic crates including `ring`, `rustls`, `argon2`, `aes-gcm`, `aes-siv`, `ccm`, `rsa`, `p256`, `p384`, `hkdf`, `pbkdf2`, `scrypt`, and `x509-parser`.

---

## 1. Top-Level Module Functions & Properties

| `node:crypto` API / Method                                            | Standard Signature                                                                           | Implementation Status | Implementation Notes                                                       |
| :-------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :-------------------: | :------------------------------------------------------------------------- |
| `crypto.argon2(algorithm, parameters, callback)`                      | `(algorithm: string, parameters: Argon2Parameters): Promise<Buffer>`                         |        ✅ Full        | Asynchronous Argon2d / Argon2i / Argon2id KDF                              |
| `crypto.argon2Sync(algorithm, parameters)`                            | `(algorithm: string, parameters: Argon2Parameters): Buffer`                                  |        ✅ Full        | Synchronous Argon2d / Argon2i / Argon2id KDF                               |
| `crypto.createHash(algorithm, options?)`                              | `(algorithm: string): Hash`                                                                  |        ✅ Full        | Creates a `Hash` instance (`sha256`, `sha512`, `sha1`, `md5`)              |
| `crypto.hash(algorithm, data, outputEncoding?)`                       | `(algorithm: string, data: Buffer\|string, outputEncoding?: string): Buffer\|string`         |        ✅ Full        | One-shot hashing helper                                                    |
| `crypto.getHashes()`                                                  | `(): string[]`                                                                               |        ✅ Full        | Returns list of supported hash digests                                     |
| `crypto.createHmac(algorithm, key, options?)`                         | `(algorithm: string, key: Buffer\|string): Hmac`                                             |        ✅ Full        | Creates an `Hmac` instance using `ring`                                    |
| `crypto.createMac(algorithm, key, options?)`                          | `(algorithm: string, key: Buffer\|string): Hmac`                                             |        ✅ Full        | MAC creation helper                                                        |
| `crypto.pbkdf2(password, salt, iterations, keylen, digest, callback)` | `(password, salt, iterations, keylen, digest): Promise<Buffer>`                              |        ✅ Full        | Async PBKDF2 using `ring`                                                  |
| `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)`       | `(password, salt, iterations, keylen, digest): Buffer`                                       |        ✅ Full        | Sync PBKDF2 using `ring`                                                   |
| `crypto.hkdf(digest, ikm, salt, info, keylen, callback)`              | `(digest, ikm, salt, info, keylen): Promise<Buffer>`                                         |        ✅ Full        | Async HKDF key derivation                                                  |
| `crypto.hkdfSync(digest, ikm, salt, info, keylen)`                    | `(digest, ikm, salt, info, keylen): Buffer`                                                  |        ✅ Full        | Sync HKDF key derivation                                                   |
| `crypto.scrypt(password, salt, keylen, options?, callback)`           | `(password, salt, keylen, options?): Promise<Buffer>`                                        |        ✅ Full        | Async Scrypt password hashing                                              |
| `crypto.scryptSync(password, salt, keylen, options?)`                 | `(password, salt, keylen, options?): Buffer`                                                 |        ✅ Full        | Sync Scrypt password hashing                                               |
| `crypto.randomBytes(size, callback?)`                                 | `(size: number): Buffer`                                                                     |        ✅ Full        | Cryptographically secure random bytes via `OsRng`                          |
| `crypto.randomBytesSync(size)`                                        | `(size: number): Buffer`                                                                     |        ✅ Full        | Sync random bytes                                                          |
| `crypto.randomFill(buffer, offset?, size?, callback?)`                | `(buffer: Buffer, offset?: number, size?: number): Buffer`                                   |        ✅ Full        | Async random fill                                                          |
| `crypto.randomFillSync(buffer, offset?, size?)`                       | `(buffer: Buffer, offset?: number, size?: number): Buffer`                                   |        ✅ Full        | Sync random fill                                                           |
| `crypto.randomInt(min, max?, callback?)`                              | `(minOrMax: number, max?: number): number`                                                   |        ✅ Full        | Uniformly distributed random integer                                       |
| `crypto.randomUUID(options?)`                                         | `(): string`                                                                                 |        ✅ Full        | RFC 4122 v4 UUID                                                           |
| `crypto.randomUUIDv7(options?)`                                       | `(): string`                                                                                 |        ✅ Full        | RFC 9562 v7 UUID with millisecond timestamp                                |
| `crypto.createECDH(curveName)`                                        | `(curveName: string): ECDH`                                                                  |        ✅ Full        | Elliptic Curve Diffie-Hellman                                              |
| `crypto.createDiffieHellman(prime, generator?)`                       | `(prime, generator?): ECDH`                                                                  |        ✅ Full        | DH key exchange initialization                                             |
| `crypto.createDiffieHellmanGroup(groupName)`                          | `(groupName: string): ECDH`                                                                  |        ✅ Full        | Predefined MODP group DH key exchange                                      |
| `crypto.generateKeyPairSync(type, options?)`                          | `(type: string, options?: RsaOptions): KeyPairResult`                                        |        ✅ Full        | Generates RSA keypair with PKCS#8 / PEM output                             |
| `crypto.publicEncrypt(key, buffer)`                                   | `(key: Buffer\|string, buffer: Buffer): Buffer`                                              |        ✅ Full        | RSA public key encryption                                                  |
| `crypto.privateDecrypt(key, buffer)`                                  | `(key: Buffer\|string, buffer: Buffer): Buffer`                                              |        ✅ Full        | RSA private key decryption                                                 |
| `crypto.publicDecrypt(key, buffer)`                                   | `(key: Buffer\|string, buffer: Buffer): Buffer`                                              |        ✅ Full        | RSA public key decryption / verification                                   |
| `crypto.privateEncrypt(key, buffer)`                                  | `(key: Buffer\|string, buffer: Buffer): Buffer`                                              |        ✅ Full        | RSA private key encryption / signing                                       |
| `crypto.sign(algorithm, data, key)`                                   | `(algorithm: string, data: Buffer\|string, key: Buffer\|string): Buffer`                     |        ✅ Full        | Calculates signature for data using private key                            |
| `crypto.verify(algorithm, data, key, signature)`                      | `(algorithm: string, data: Buffer\|string, key: Buffer\|string, signature: Buffer): boolean` |        ✅ Full        | Verifies signature for data using public key                               |
| `crypto.createSign(algorithm)`                                        | `(algorithm: string): Sign`                                                                  |        ✅ Full        | Creates a `Sign` stream/class                                              |
| `crypto.createVerify(algorithm)`                                      | `(algorithm: string): Verify`                                                                |        ✅ Full        | Creates a `Verify` stream/class                                            |
| `crypto.createPublicKey(key)`                                         | `(key: Buffer\|string): KeyObject`                                                           |        ✅ Full        | Creates public `KeyObject`                                                 |
| `crypto.createPrivateKey(key)`                                        | `(key: Buffer\|string): KeyObject`                                                           |        ✅ Full        | Creates private `KeyObject`                                                |
| `crypto.createSecretKey(key, encoding?)`                              | `(key: Buffer\|string, encoding?: string): KeyObject`                                        |        ✅ Full        | Creates secret `KeyObject`                                                 |
| `crypto.encapsulate(key)`                                             | `(key: Buffer\|string): EncapsulateResult`                                                   |        ✅ Full        | KEM Key Encapsulation                                                      |
| `crypto.decapsulate(key, ciphertext)`                                 | `(key: Buffer\|string, ciphertext: Buffer): Buffer`                                          |        ✅ Full        | KEM Key Decapsulation                                                      |
| `crypto.createTls(provider?)`                                         | `(provider?: string): TLS`                                                                   |        ✅ Full        | Creates `TLS` context with provider (`ring`, `openssl`, `btls`, `mbedtls`) |

---

## 2. Classes & Methods

### `Class: Hash`

- `update(data: Buffer | string, encoding?: string): Hash`
- `digest(outputEncoding?: string): Buffer | string`

### `Class: Hmac`

- `update(data: Buffer | string, encoding?: string): Hmac`
- `digest(outputEncoding?: string): Buffer | string`

### `Class: ECDH` / `Ecdh`

- `generateKeys(encoding?, format?): Buffer | string`
- `computeSecret(otherPublicKey: Buffer | string, inputEncoding?, outputEncoding?): Buffer | string`
- `getPrivateKey(encoding?): Buffer | string`
- `setPrivateKey(privateKey: Buffer | string, encoding?): void`
- `getPublicKey(encoding?, format?): Buffer | string`
- `setPublicKey(publicKey: Buffer | string, encoding?): void`

### `Class: KeyObject`

- `type: string` ('secret' | 'public' | 'private')
- `asymmetricKeyType: string | null`
- `export(format?: string): Buffer | string`

### `Class: CryptoKeyPair`

- `publicKey: KeyObject`
- `privateKey: KeyObject`

### `Class: Sign`

- `update(data: Buffer | string, encoding?: string): Sign`
- `sign(key: Buffer | string): Buffer`

### `Class: Verify`

- `update(data: Buffer | string, encoding?: string): Verify`
- `verify(key: Buffer | string, signature: Buffer): boolean`

### `Class: X509Certificate`

- `new X509Certificate(buffer: Buffer | string)`
- `raw: Buffer`
- `subject: string`
- `issuer: string`
- `serialNumber: string`
- `validFrom: string`, `validTo: string`
- `signatureAlgorithm: string`, `signatureAlgorithmOid: string`
- `ca: boolean`
- `fingerprint: string`, `fingerprint256: string`, `fingerprint512: string`
- `checkEmail(email: string): boolean`
- `checkHost(name: string): string | null`
- `checkIP(ip: string): string | null`
- `checkIssued(otherCert: X509Certificate): boolean`
- `checkPrivateKey(privateKey: KeyObject): boolean`
- `verify(publicKey: KeyObject): boolean`
- `toString(): string`, `toJSON(): string`

### `Class: TLS`

- `new TLS(provider?: string)`
- `provider: string` (Defaults to `"ring"`, supports `"openssl"` via `rustls-openssl`, `"btls"`, `"mbedtls"`)
- `isActive(): boolean`
- `createClientConfig(): string`

---

## 3. AEAD Ciphers

- `encryptAead(algorithm: string, key: Buffer, iv: Buffer, plaintext: Buffer, aad?: Buffer): AeadEncryptResult`
- `decryptAead(algorithm: string, key: Buffer, iv: Buffer, ciphertext: Buffer, authTag: Buffer, aad?: Buffer): Buffer`
- Supported modes:
  - `aes-128-gcm`, `aes-256-gcm`
  - `aes-128-siv`, `aes-256-siv`
  - `aes-128-ccm`
