import { expect, test } from "vite-plus/test";
import { Ksuid, KsuidMs, generateKsuid, generateKsuidMs } from "./main.ts";

test("Ksuid - create and convert Base62", () => {
  const ksuid = Ksuid.now();
  const base62 = ksuid.toString();
  expect(base62).toHaveLength(27);
  expect(ksuid.toBase62()).toBe(base62);

  const bytes = ksuid.bytes();
  expect(bytes).toHaveLength(20);

  const payload = ksuid.payload();
  expect(payload).toHaveLength(16);

  expect(ksuid.timestampSeconds()).toBeGreaterThan(0);

  // From base62
  const parsed = Ksuid.fromBase62(base62);
  expect(parsed.toString()).toBe(base62);
  expect(parsed.equals(ksuid)).toBe(true);

  // From constructor with string
  const fromCtorString = new Ksuid(base62);
  expect(fromCtorString.toString()).toBe(base62);

  // From constructor with bytes
  const fromCtorBytes = new Ksuid(bytes);
  expect(fromCtorBytes.toString()).toBe(base62);

  // From bytes
  const fromBytes = Ksuid.fromBytes(bytes);
  expect(fromBytes.toString()).toBe(base62);
});

test("Ksuid - Crockford Base32 encoding and custom alphabets", () => {
  const ksuid = Ksuid.now();

  // Standard Crockford Base32
  const c32 = ksuid.toCrockfordBase32();
  expect(c32).toHaveLength(32);
  expect(ksuid.toBase32()).toBe(c32);
  expect(ksuid.toString({ enc: "crockford" })).toBe(c32);
  expect(ksuid.toString("crockford")).toBe(c32);

  // Roundtrip standard Crockford
  const parsedC32 = Ksuid.fromCrockfordBase32(c32);
  expect(parsedC32.equals(ksuid)).toBe(true);

  const parsedFromCtor = new Ksuid(c32, { enc: "crockford" });
  expect(parsedFromCtor.equals(ksuid)).toBe(true);

  // Custom Crockford Alphabet (shuffled 32 characters)
  const custom32Alphabet = "ZYXWVUTSRQPONMKJHGFEDCBA98765432";
  const customC32 = ksuid.toCrockfordBase32(custom32Alphabet);
  expect(customC32).toHaveLength(32);
  expect(customC32).not.toBe(c32);

  const parsedCustomC32 = Ksuid.fromCrockfordBase32(customC32, custom32Alphabet);
  expect(parsedCustomC32.equals(ksuid)).toBe(true);

  const parsedCustomC32Opts = new Ksuid(customC32, {
    enc: "crockford",
    alphabet: custom32Alphabet,
  });
  expect(parsedCustomC32Opts.equals(ksuid)).toBe(true);

  // Custom Base62 Alphabet (shuffled 62 characters)
  const custom62Alphabet = "zyxwvutsrqponmlkjihgfedcbaZYXWVUTSRQPONMLKJIHGFEDCBA9876543210";
  const customB62 = ksuid.toBase62(custom62Alphabet);
  expect(customB62).toHaveLength(27);

  const parsedCustomB62 = Ksuid.fromBase62(customB62, custom62Alphabet);
  expect(parsedCustomB62.equals(ksuid)).toBe(true);
});

test("Ksuid - comparison", () => {
  const ksuid1 = Ksuid.fromSeconds(1_555_555_555);
  const ksuid2 = Ksuid.fromSeconds(1_777_777_777);

  expect(ksuid1.compare(ksuid2)).toBe(-1);
  expect(ksuid2.compare(ksuid1)).toBe(1);
  expect(ksuid1.compare(ksuid1)).toBe(0);
  expect(ksuid1.equals(ksuid1)).toBe(true);
  expect(ksuid1.equals(ksuid2)).toBe(false);
});

test("KsuidMs - Crockford Base32 and custom alphabets", () => {
  const ksuidMs = KsuidMs.now();

  const c32 = ksuidMs.toCrockfordBase32();
  expect(c32).toHaveLength(32);
  expect(ksuidMs.toString({ enc: "base32" })).toBe(c32);

  const parsed = KsuidMs.fromCrockfordBase32(c32);
  expect(parsed.equals(ksuidMs)).toBe(true);

  // Custom alphabet
  const custom32Alphabet = "abcdefghijklmnopqrstuvwxyz234567";
  const customC32 = ksuidMs.toCrockfordBase32(custom32Alphabet);
  const parsedCustom = KsuidMs.fromCrockfordBase32(customC32, custom32Alphabet);
  expect(parsedCustom.equals(ksuidMs)).toBe(true);
});

test("generateKsuid and generateKsuidMs with options", () => {
  const k1 = generateKsuid(undefined, { enc: "crockford" });
  expect(k1).toHaveLength(32);

  const k2 = generateKsuidMs(undefined, "crockford");
  expect(k2).toHaveLength(32);
});
