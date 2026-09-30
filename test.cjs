const assert = require("node:assert/strict");
const test = require("node:test");

void test("ksuid native addon works in CJS", async () => {
  const { Ksuid, KsuidMs } = await import("./index.js");

  const k1 = Ksuid.fromSeconds(1_555_555_555);
  const k2 = Ksuid.fromSeconds(1_777_777_777);

  assert.equal(k1.compare(k2), -1);
  assert.equal(k2.compare(k1), 1);
  assert.equal(k1.equals(k1), true);

  const base62 = k1.toString();
  assert.equal(base62.length, 27);
  assert.equal(k1.toBase62(), base62);

  const parsed = Ksuid.fromBase62(base62);
  assert.equal(parsed.toString(), base62);

  const kms = KsuidMs.now();
  assert.equal(kms.toString().length, 27);
  assert.equal(kms.payload().length, 15);
});
