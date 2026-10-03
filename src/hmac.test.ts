import assert from "node:assert/strict";
import { describe, it } from "vitest";
import { Hmac, createHmac } from "../dist/index.js";

describe("hmac", () => {
  it("Hmac class and createHmac", () => {
    const secret = "secret-key";
    const data = "hello world";

    const hmac1 = createHmac("sha256", secret);
    hmac1.update(data);
    const hex1 = hmac1.digest("hex");

    const hmac2 = new Hmac("sha256", secret);
    hmac2.update(data);
    const hex2 = hmac2.digest("hex");

    assert.equal(hex1, hex2);
    assert.equal(typeof hex1, "string");
    assert.equal(hex1.length, 64);
  });

  it("Hmac buffer and base64 digest", () => {
    const hmac = createHmac("sha256", "key");
    hmac.update("data");
    const buf = hmac.digest("buffer");
    assert.ok(Buffer.isBuffer(buf));

    const hmacB64 = createHmac("sha256", "key");
    hmacB64.update("data");
    const b64 = hmacB64.digest("base64");
    assert.equal(b64, buf.toString("base64"));
  });
});
