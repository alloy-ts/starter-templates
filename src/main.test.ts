import { expect, test } from "vite-plus/test";
import { Ksuid, KsuidMs, generateKsuid, generateKsuidMs } from "./main.ts";

test("Ksuid - create and convert", () => {
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

test("Ksuid - comparison", () => {
  const ksuid1 = Ksuid.fromSeconds(1_555_555_555);
  const ksuid2 = Ksuid.fromSeconds(1_777_777_777);

  expect(ksuid1.compare(ksuid2)).toBe(-1);
  expect(ksuid2.compare(ksuid1)).toBe(1);
  expect(ksuid1.compare(ksuid1)).toBe(0);
  expect(ksuid1.equals(ksuid1)).toBe(true);
  expect(ksuid1.equals(ksuid2)).toBe(false);
});

test("KsuidMs - create and convert", () => {
  const ksuidMs = KsuidMs.now();
  const base62 = ksuidMs.toString();
  expect(base62).toHaveLength(27);

  const bytes = ksuidMs.bytes();
  expect(bytes).toHaveLength(20);

  const payload = ksuidMs.payload();
  expect(payload).toHaveLength(15);

  expect(ksuidMs.timestampMillis()).toBeGreaterThan(0);

  const parsed = KsuidMs.fromBase62(base62);
  expect(parsed.toString()).toBe(base62);
  expect(parsed.equals(ksuidMs)).toBe(true);
});

test("KsuidMs - comparison", () => {
  const k1 = KsuidMs.fromMillis(1_555_555_555_000);
  const k2 = KsuidMs.fromMillis(1_777_777_777_000);

  expect(k1.compare(k2)).toBe(-1);
  expect(k2.compare(k1)).toBe(1);
  expect(k1.equals(k1)).toBe(true);
});

test("generateKsuid and generateKsuidMs helpers", () => {
  const k1 = generateKsuid();
  expect(k1).toHaveLength(27);

  const k2 = generateKsuidMs();
  expect(k2).toHaveLength(27);
});
