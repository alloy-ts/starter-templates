# `@lib/ksuid`

High-performance native Node.js addon for **KSUID** (K-Sortable Unique Identifier) powered by Rust (`svix-ksuid`).

Supports:

- Standard **32-bit (4-byte)** second epoch (`Ksuid`)
- **48-bit (6-byte)** millisecond epoch (UUIDv7-style)
- **64-bit (8-byte)** millisecond epoch (`KsuidMs`)
- Multiple encodings: **Base62**, **Crockford Base32**, and **Base36**
- Custom & shuffled alphabets
- Instance-level default encoding configurations via `options.enc` and `options.alphabet`

---

## Installation

```bash
npm install @lib/ksuid
```

---

## Quick Start

```typescript
import { Ksuid, KsuidMs, generateKsuid } from "@lib/ksuid";

// Generate standard 32-bit KSUID (Base62 encoded)
const ksuid = Ksuid.now();
console.log(ksuid.toString()); // e.g. "1srOrx2ZWZBpBUvZwXKQmoEYga2"

// Generate 48-bit (6-byte) millisecond-precision KSUID
const ksuid48 = new Ksuid(undefined, { timestampSize: "48bit" });
console.log(ksuid48.timestampMillis()); // e.g. 1730000000000
console.log(ksuid48.payload()); // 14-byte Uint8Array

// Generate 64-bit (8-byte) millisecond-precision KSUID
const ksuidMs = KsuidMs.now();
console.log(ksuidMs.timestampMillis());
console.log(ksuidMs.payload()); // 12-byte Uint8Array
```

---

## Encodings & Custom Alphabets

### Default Encoding Configuration

You can configure default encoding (`enc`) and custom `alphabet` in the constructor:

```typescript
// Configure instance to default to Crockford Base32
const ksuid32 = new Ksuid(undefined, { enc: "crockford" });
console.log(ksuid32.toString()); // Output is 32-character Crockford Base32 string!

// Configure instance with a custom shuffled Crockford alphabet
const customAlphabet = "ZYXWVUTSRQPONMKJHGFEDCBA98765432";
const ksuidCustom = new Ksuid(undefined, { enc: "crockford", alphabet: customAlphabet });
const str = ksuidCustom.toString(); // Uses custom alphabet

// Parse back using the same custom options
const parsed = new Ksuid(str, { enc: "crockford", alphabet: customAlphabet });
console.log(parsed.equals(ksuidCustom)); // true
```

### Supported Encodings

1. **Base62** (default)
   - `ksuid.toBase62(alphabet?)`
   - `Ksuid.fromBase62(str, alphabet?)`

2. **Crockford Base32** (`"crockford"` or `"base32"`)
   - `ksuid.toCrockfordBase32(alphabet?)` or `ksuid.toBase32(alphabet?)`
   - `Ksuid.fromCrockfordBase32(str, alphabet?)` or `Ksuid.fromBase32(str, alphabet?)`

3. **Base36** (`"base36"`)
   - `ksuid.toBase36(alphabet?)`
   - `Ksuid.fromBase36(str, alphabet?)`

---

## Timestamp Modes (`timestampSize`)

- `"32bit"` / `"4bytes"` / `32` / `4`: 32-bit second epoch timestamp + 16-byte payload (default for `Ksuid`).
- `"48bit"` / `"6bytes"` / `48` / `6`: 48-bit millisecond epoch timestamp + 14-byte payload.
- `"64bit"` / `"8bytes"` / `64` / `8`: 64-bit millisecond epoch timestamp + 12-byte payload (default for `KsuidMs`).

```typescript
const k32 = new Ksuid(undefined, { timestampSize: "32bit" });
console.log(k32.payload().length); // 16 bytes

const k48 = new Ksuid(undefined, { timestampSize: "48bit" });
console.log(k48.payload().length); // 14 bytes

const k64 = new Ksuid(undefined, { timestampSize: "64bit" });
console.log(k64.payload().length); // 12 bytes
```

---

## Development & Testing

```bash
# Run tests
npm run test

# Run build
npm run build

# Run formatting & checks
npm run fmt
npm run check
```
