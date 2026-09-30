const assert = require("node:assert/strict");
const test = require("node:test");

void test("ksuid native addon works with Crockford Base32 and custom alphabets in CJS", async () => {
  const { Ksuid, KsuidMs } = await import("./index.js");

  const k1 = Ksuid.fromSeconds(1_555_555_555);
  const k2 = Ksuid.fromSeconds(1_777_777_777);

  assert.equal(k1.compare(k2), -1);
  assert.equal(k2.compare(k1), 1);
  assert.equal(k1.equals(k1), true);

  // Crockford Base32
  const c32 = k1.toCrockfordBase32();
  assert.equal(c32.length, 32);
  assert.equal(k1.toString({ enc: "crockford" }), c32);

  const parsedC32 = Ksuid.fromCrockfordBase32(c32);
  assert.equal(parsedC32.equals(k1), true);

  // Custom Crockford alphabet
  const customAlphabet = "ZYXWVUTSRQPONMKJHGFEDCBA98765432";
  const customC32 = k1.toCrockfordBase32(customAlphabet);
  assert.equal(customC32.length, 32);
  assert.notEqual(customC32, c32);

  const parsedCustom = Ksuid.fromCrockfordBase32(customC32, customAlphabet);
  assert.equal(parsedCustom.equals(k1), true);

  const kms = KsuidMs.now();
  const kmsC32 = kms.toCrockfordBase32();
  assert.equal(kmsC32.length, 32);
  assert.equal(KsuidMs.fromCrockfordBase32(kmsC32).equals(kms), true);
});
