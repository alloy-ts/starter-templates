import assert from "node:assert/strict";
import { describe, it } from "vitest";
import { TLS } from "../dist/index.js";

describe("tls", () => {
  it("TLS default provider (ring)", () => {
    const tls = new TLS();
    assert.equal(tls.provider, "ring");
    assert.equal(tls.isAvailable(), true);
    assert.ok(tls.supportedCiphers().length > 0);
  });

  it("TLS openssl provider handling", () => {
    assert.throws(() => {
      new TLS("openssl");
    }, /OpenSSL provider conflicts with BoringSSL symbol table at runtime/);
  });

  it("TLS btls provider", () => {
    const tls = new TLS("btls");
    assert.equal(tls.provider, "btls");
    assert.equal(tls.isAvailable(), true);
    assert.ok(tls.supportedCiphers().length > 0);
  });
});
