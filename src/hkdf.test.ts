import { expect, test } from "vite-plus/test";
import { hkdf, hkdfSync } from "./main.ts";

test("hkdfSync derives key", () => {
  const derived = hkdfSync("sha256", "ikm", "salt", "info", 32);
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(32);
});

test("hkdf async derives key", async () => {
  const derived = await hkdf("sha512", "ikm", "salt", "info", 64);
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(64);
});
