import assert from "node:assert/strict";
import { describe, it } from "vitest";
import {
  argon2Hash,
  argon2HashSync,
  argon2HashRawSync,
  argon2VerifySync,
  argon2ParseOptions,
  Algorithm,
  Version,
} from "../dist/index.js";

describe("argon2", () => {
  it("argon2HashSync and argon2VerifySync", () => {
    const pwd = "myPassword123";
    const hash = argon2HashSync(pwd);
    assert.ok(hash.startsWith("$argon2id$"));

    const verified = argon2VerifySync(hash, pwd);
    assert.equal(verified, true);

    const wrongVerified = argon2VerifySync(hash, "wrongPassword");
    assert.equal(wrongVerified, false);

    const parsed = argon2ParseOptions(hash);
    assert.equal(parsed.algorithm, Algorithm.Argon2id);
    assert.equal(parsed.version, Version.V0x13);
  });

  it("argon2Hash async", async () => {
    const pwd = "asyncPassword";
    const hash = await argon2Hash(pwd);
    assert.ok(hash.startsWith("$argon2id$"));
  });

  it("argon2HashRawSync", () => {
    const pwd = "rawPassword";
    const raw = argon2HashRawSync(pwd, {
      outputLen: 16,
      salt: Buffer.from("1234567890123456"),
    });
    assert.equal(raw.length, 16);
  });
});
