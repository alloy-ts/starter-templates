import assert from "node:assert/strict";
import { describe, it } from "vitest";
import { pbkdf2, pbkdf2Sync } from "../dist/index.js";

describe("pbkdf2", () => {
  it("pbkdf2Sync", () => {
    const key = pbkdf2Sync("secret", "salt", 1000, 32, "sha256");
    assert.ok(Buffer.isBuffer(key));
    assert.equal(key.length, 32);
  });

  it("pbkdf2 async", async () => {
    const key = await pbkdf2("secret", "salt", 1000, 32, "sha256");
    assert.ok(Buffer.isBuffer(key));
    assert.equal(key.length, 32);

    const keySync = pbkdf2Sync("secret", "salt", 1000, 32, "sha256");
    assert.equal(key.toString("hex"), keySync.toString("hex"));
  });
});
