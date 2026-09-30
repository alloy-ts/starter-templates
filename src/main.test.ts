import { expect, test } from "vite-plus/test";
import {
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
} from "../index.js";

test("serialize and deserialize BSON document", () => {
  const doc = { name: "Bob", age: 25, active: true };
  const bytes = serialize(doc);
  expect(Buffer.isBuffer(bytes)).toBe(true);

  const restored = deserialize(bytes);
  expect(restored).toEqual(doc);
});

test("toBson and fromBson extended JSON conversions", () => {
  const obj = { foo: "bar" };
  const bsonVal = toBson(obj);
  expect(bsonVal).toBeDefined();

  const restored = fromBson(bsonVal);
  expect(restored).toEqual(obj);
});

test("ObjectId, BsonDateTime, and Binary helpers", () => {
  const oid = new ObjectId();
  expect(oid.toHex().length).toBe(24);

  const dt = new BsonDateTime(1700000000000);
  expect(dt.toMillis()).toBe(1700000000000);

  const bin = new Binary(Buffer.from([1, 2, 3]), 0);
  expect(bin.subType).toBe(0);
  expect(Array.from(bin.bytes)).toEqual([1, 2, 3]);
});

test("Timestamp, Regex, MinKey, MaxKey, Code helpers", () => {
  const ts = new Timestamp(50, 2);
  expect(ts.time).toBe(50);
  expect(ts.increment).toBe(2);

  const regex = new Regex("abc", "g");
  expect(regex.pattern).toBe("abc");
  expect(regex.options).toBe("g");

  expect(new MinKey()).toBeDefined();
  expect(new MaxKey()).toBeDefined();

  const code = new Code("console.log(1)");
  expect(code.code).toBe("console.log(1)");
});
