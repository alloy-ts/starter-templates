import { expect, test } from "vite-plus/test";
import { scrypt, scryptSync } from "./main.ts";

test("scryptSync derives key", () => {
  const derived = scryptSync("password", "salt", 32, {
    cost: 1024,
    blockSize: 8,
    parallelization: 1,
  });
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(32);
});

test("scrypt async derives key", async () => {
  const derived = await scrypt("password", "salt", 64, {
    cost: 1024,
    blockSize: 8,
    parallelization: 1,
  });
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(64);
});
