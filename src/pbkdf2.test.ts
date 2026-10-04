import { expect, test } from "vite-plus/test";
import { pbkdf2Sync } from "../index.js";

test("pbkdf2Sync derives key", () => {
  const password = Buffer.from("secret");
  const salt = Buffer.from("salt12345678");
  const key = pbkdf2Sync(password, salt, 1000, 32, "sha256");
  expect(Buffer.isBuffer(key)).toBe(true);
  expect(key.length).toBe(32);
});
