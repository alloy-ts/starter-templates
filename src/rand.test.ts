import { expect, test } from "vite-plus/test";
import {
  randomBytes,
  randomBytesSync,
  randomFill,
  randomFillSync,
  randomInt,
  randomUUID,
  randomUUIDv7,
} from "./main.ts";

test("randomBytes and randomBytesSync", () => {
  const syncBytes = randomBytesSync(16);
  expect(Buffer.isBuffer(syncBytes)).toBe(true);
  expect(syncBytes.length).toBe(16);

  const asyncBytes = randomBytes(32);
  expect(Buffer.isBuffer(asyncBytes)).toBe(true);
  expect(asyncBytes.length).toBe(32);
});

test("randomFill and randomFillSync", () => {
  const buf = Buffer.alloc(10);
  const filled = randomFillSync(buf, 0, 10);
  expect(filled.equals(buf)).toBe(true);

  const buf2 = Buffer.alloc(10);
  const filled2 = randomFill(buf2, 0, 10);
  expect(filled2.equals(buf2)).toBe(true);
});

test("randomInt", () => {
  const num = randomInt(1, 10);
  expect(num).toBeGreaterThanOrEqual(1);
  expect(num).toBeLessThan(10);
});

test("randomUUID and randomUUIDv7", () => {
  const uuid = randomUUID();
  expect(typeof uuid).toBe("string");
  expect(uuid.length).toBe(36);

  const uuidv7 = randomUUIDv7();
  expect(typeof uuidv7).toBe("string");
  expect(uuidv7.length).toBe(36);
});
