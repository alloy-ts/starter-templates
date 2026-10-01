const assert = require("node:assert/strict");
const test = require("node:test");

void test("ksuid native addon works with Crockford Base32, Base36, custom alphabets, enc, and timestampSize in CJS", async () => {
  const { Ksuid, KsuidMs } = await import("./index.js");

  const k1 = Ksuid.fromSeconds(1_555_555_555);
  const k2 = Ksuid.fromSeconds(1_777_777_777);

  assert.equal(k1.compare(k2), -1);
  assert.equal(k2.compare(k1), 1);
  assert.equal(k1.equals(k1), true);

  // Instance default enc
  const kCrockford = new Ksuid(undefined, { enc: "crockford" });
  const c32 = kCrockford.toString();
  assert.equal(c32.length, 32);

  // Base36
  const b36 = k1.toBase36();
  assert.equal(b36.length, 31);
  assert.equal(Ksuid.fromBase36(b36).equals(k1), true);

  // timestampSize options
  const k32 = new Ksuid(undefined, { timestampSize: "32bit" });
  assert.equal(k32.payload().length, 16);

  const k48 = new Ksuid(undefined, { timestampSize: "48bit" });
  assert.equal(k48.payload().length, 14);

  const k64 = new Ksuid(undefined, { timestampSize: "64bit" });
  assert.equal(k64.payload().length, 12);

  const kms = KsuidMs.now();
  const kmsC32 = kms.toCrockfordBase32();
  assert.equal(kmsC32.length, 32);
  assert.equal(KsuidMs.fromCrockfordBase32(kmsC32).equals(kms), true);
});
