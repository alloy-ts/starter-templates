const assert = require("node:assert/strict");
const test = require("node:test");

const { encode, decode } = require("./index.js");

test("encode and decode message pack basic object", () => {
  const obj = { compact: true, schema: 0, foo: "bar", numbers: [1, 2, 3] };
  const encoded = encode(obj);
  assert.ok(encoded instanceof Uint8Array);
  const decoded = decode(encoded);
  assert.deepEqual(decoded, obj);
});

test("encode and decode various data types", () => {
  const cases = [
    null,
    true,
    false,
    0,
    12345,
    -999,
    3.14159,
    "hello world",
    [null, true, 42, "nested"],
    { a: 1, b: "two", c: [3, 4] },
  ];
  for (const item of cases) {
    const encoded = encode(item);
    assert.ok(encoded instanceof Uint8Array);
    const decoded = decode(encoded);
    assert.deepEqual(decoded, item);
  }
});

test("decode invalid message pack throws error", () => {
  const invalidData = new Uint8Array([0xc1]); // 0xc1 is never used byte in MessagePack
  assert.throws(() => {
    decode(invalidData);
  });
});
