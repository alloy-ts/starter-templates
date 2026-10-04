import { expect, test } from "vite-plus/test";
import { createHash, getHashes } from "../index.js";

test("getHashes returns supported hashes", () => {
  const hashes = getHashes();
  expect(Array.isArray(hashes)).toBe(true);
  expect(hashes).toContain("sha256");
});

test("createHash calculates sha256 digest", () => {
  const hasher = createHash("sha256");
  hasher.update(Buffer.from("hello world"));
  const hex = hasher.digest("hex");
  expect(hex).toBe("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
});
