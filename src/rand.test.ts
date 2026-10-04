import { expect, test } from "vite-plus/test";
import { randomBytes, randomInt, randomUuid, randomUuidV7 } from "../index.js";

test("randomBytes generates buffer of requested length", () => {
  const buf = randomBytes(16);
  expect(Buffer.isBuffer(buf)).toBe(true);
  expect(buf.length).toBe(16);
});

test("randomInt generates integer in range", () => {
  const n = randomInt(1, 10);
  expect(n).toBeGreaterThanOrEqual(1);
  expect(n).toBeLessThan(10);
});

test("randomUuid and randomUuidV7 return UUID strings", () => {
  const u4 = randomUuid();
  const u7 = randomUuidV7();
  expect(typeof u4).toBe("string");
  expect(typeof u7).toBe("string");
  expect(u4.length).toBe(36);
  expect(u7.length).toBe(36);
});
