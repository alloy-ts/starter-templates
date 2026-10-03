import assert from "node:assert/strict";
import { describe, it } from "vitest";
import { Hash, createHash, hash, getHashes } from "../dist/index.js";

describe("crypto_hasher", () => {
  it("Hash class and createHash", () => {
    const h = createHash("sha256");
    h.update("hello world");
    const hex = h.digest("hex");
    assert.equal(
      hex,
      "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
  });

  it("one-shot hash function", () => {
    const hex = hash("sha256", "hello world", "hex");
    assert.equal(
      hex,
      "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
  });

  it("getHashes", () => {
    const hashes = getHashes();
    assert.ok(hashes.includes("sha256"));
    assert.ok(hashes.includes("sha512"));
  });
});
