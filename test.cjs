const assert = require("node:assert/strict");
const test = require("node:test");

const {
  serialize,
  deserialize,
  toBson,
  fromBson,
  ObjectId,
  BsonDateTime,
  Binary,
  Timestamp,
  Regex,
  MinKey,
  MaxKey,
  Code,
} = require("./index.js");

test("serialize and deserialize BSON document", () => {
  const doc = { name: "Alice", age: 30, tags: ["rust", "node"] };
  const bytes = serialize(doc);
  assert.ok(Buffer.isBuffer(bytes));
  assert.ok(bytes.length > 0);

  const restored = deserialize(bytes);
  assert.deepEqual(restored, doc);
});

test("toBson and fromBson extended JSON conversions", () => {
  const obj = { key: "value", num: 42 };
  const bsonVal = toBson(obj);
  assert.ok(bsonVal);

  const restored = fromBson(bsonVal);
  assert.deepEqual(restored, obj);
});

test("ObjectId helper class", () => {
  const oid1 = new ObjectId();
  assert.equal(oid1.toHex().length, 24);
  assert.equal(oid1.toString(), oid1.toHex());
  assert.ok(typeof oid1.getTimestamp() === "number");

  const hex = "507f191e810c19729de860ea";
  const oid2 = new ObjectId(hex);
  assert.equal(oid2.toHex(), hex);
});

test("BsonDateTime helper class", () => {
  const nowMs = Date.now();
  const dt = new BsonDateTime(nowMs);
  assert.equal(dt.toMillis(), nowMs);

  const iso = "2024-01-01T00:00:00.000Z";
  const dtIso = BsonDateTime.fromDateString(iso);
  assert.equal(dtIso.toIsoString().slice(0, 10), "2024-01-01");
});

test("Binary helper class", () => {
  const buf = Buffer.from("hello binary");
  const bin = new Binary(buf, 0);
  assert.equal(bin.subType, 0);
  assert.deepEqual(bin.bytes, buf);
});

test("Timestamp, Regex, MinKey, MaxKey, Code helper classes", () => {
  const ts = new Timestamp(100, 1);
  assert.equal(ts.time, 100);
  assert.equal(ts.increment, 1);

  const regex = new Regex("^hello", "i");
  assert.equal(regex.pattern, "^hello");
  assert.equal(regex.options, "i");

  const minKey = new MinKey();
  assert.ok(minKey);

  const maxKey = new MaxKey();
  assert.ok(maxKey);

  const code = new Code("function() { return true; }");
  assert.equal(code.code, "function() { return true; }");
});
