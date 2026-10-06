import { expect, test } from "vite-plus/test";
import { pbkdf2, pbkdf2Sync } from "./main.ts";

test("pbkdf2Sync derives key", () => {
  const derived = pbkdf2Sync("password", "salt", 1000, 32, "sha256");
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(32);
});

test("pbkdf2 async derives key", async () => {
  const derived = await pbkdf2("password", "salt", 1000, 64, "sha512");
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(64);
});
