import { expect, test } from "vite-plus/test";
import { argon2, argon2Sync } from "./main.ts";

test("argon2Sync generates key with standard node:crypto parameter signature", () => {
  const parameters = {
    message: "password",
    nonce: "randomNonce123456",
    parallelism: 1,
    tagLength: 32,
    memory: 4096,
    passes: 3,
  };

  const derived = argon2Sync("argon2id", parameters);
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(32);
});

test("argon2 async generates key", async () => {
  const parameters = {
    message: "password",
    nonce: "randomNonce123456",
    parallelism: 1,
    tagLength: 32,
    memory: 4096,
    passes: 2,
  };

  const derived = await argon2("argon2id", parameters);
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(32);
});
