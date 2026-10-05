import { expect, test } from "vite-plus/test";
import { createHash, getHashes, hash, Hash } from "./main.ts";

test("getHashes returns supported algorithms", () => {
  const hashes = getHashes();
  expect(hashes).toContain("sha256");
  expect(hashes).toContain("sha512");
  expect(hashes).toContain("sha1");
  expect(hashes).toContain("md5");
});

test("createHash sha256 hex digest", () => {
  const h = createHash("sha256");
  h.update("hello");
  const digest = h.digest("hex");
  expect(digest).toBe("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
});

test("one-shot hash function", () => {
  const res = hash("sha256", "hello", "hex");
  expect(res).toBe("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
});

test("Hash class directly", () => {
  const h = new Hash("md5");
  h.update("test");
  const digest = h.digest("hex");
  expect(digest).toBe("098f6bcd4621d373cade4e832627b4f6");
});
