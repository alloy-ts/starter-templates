import { expect, test } from "vite-plus/test";
import { argon2Sync } from "../index.js";

test("argon2Sync hashes password and salt", () => {
  const password = Buffer.from("my-password");
  const salt = Buffer.from("somesalt12345678");
  const hash = argon2Sync(password, salt, {
    memoryCost: 4096,
    timeCost: 1,
    parallelism: 1,
    outputLength: 32,
  });
  expect(Buffer.isBuffer(hash)).toBe(true);
  expect(hash.length).toBe(32);
});
