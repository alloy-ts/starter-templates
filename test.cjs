const assert = require("node:assert/strict");
const test = require("node:test");
const Sqids = require("./index.js");
const { defaultOptions } = require("./index.js");

void test("Simple encode & decode", () => {
  const sqids = new Sqids();
  const id = sqids.encode([1, 2, 3]);
  assert.equal(id, "86Rf07");
  const numbers = sqids.decode(id);
  assert.deepEqual(numbers, [1, 2, 3]);
});

void test("Enforce minimum length for IDs", () => {
  const sqids = new Sqids({ minLength: 10 });
  const id = sqids.encode([1, 2, 3]);
  assert.equal(id, "86Rf07xd4z");
  const numbers = sqids.decode(id);
  assert.deepEqual(numbers, [1, 2, 3]);
});

void test("Randomize IDs by providing a custom alphabet", () => {
  const sqids = new Sqids({
    alphabet: "FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE",
  });
  const id = sqids.encode([1, 2, 3]);
  assert.equal(id, "B4aajs");
  const numbers = sqids.decode(id);
  assert.deepEqual(numbers, [1, 2, 3]);
});

void test("Prevent specific words from appearing anywhere with Set blocklist", () => {
  const sqids = new Sqids({
    blocklist: new Set(["86Rf07"]),
  });
  const id = sqids.encode([1, 2, 3]);
  assert.equal(id, "se8ojk");
  const numbers = sqids.decode(id);
  assert.deepEqual(numbers, [1, 2, 3]);
});

void test("defaultOptions export", () => {
  assert.equal(
    defaultOptions.alphabet,
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
  );
  assert.equal(defaultOptions.minLength, 0);
  assert.ok(defaultOptions.blocklist instanceof Set);
  assert.ok(defaultOptions.blocklist.has("0rgasm"));
});

void test("Validation errors", () => {
  assert.throws(
    () => new Sqids({ alphabet: "😃😃😃" }),
    /Alphabet cannot contain multibyte characters/,
  );
  assert.throws(() => new Sqids({ alphabet: "ab" }), /Alphabet length must be at least 3/);
  assert.throws(() => new Sqids({ alphabet: "aab" }), /Alphabet must contain unique characters/);
  assert.throws(() => new Sqids({ minLength: -1 }), /Minimum length has to be between 0 and 255/);
  assert.throws(() => new Sqids({ minLength: 300 }), /Minimum length has to be between 0 and 255/);

  const sqids = new Sqids();
  assert.throws(
    () => sqids.encode([-1]),
    /Encoding supports numbers between 0 and 9007199254740991/,
  );
});
