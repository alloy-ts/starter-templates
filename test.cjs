const assert = require("node:assert/strict");
const test = require("node:test");

const { parse, stringify, decode, encode } = require("./index.js");

void test("parses TOML string", () => {
  const tomlStr = 'title = "TOML Example"\n\n[owner]\nname = "Tom"';
  const parsed = parse(tomlStr);
  assert.equal(parsed.title, "TOML Example");
  assert.equal(parsed.owner.name, "Tom");
});

void test("decodes TOML Buffer", () => {
  const buf = Buffer.from('key = "value"');
  const decoded = decode(buf);
  assert.equal(decoded.key, "value");
});

void test("stringifies object to TOML", () => {
  const obj = { title: "TOML Example", owner: { name: "Tom" } };
  const str = stringify(obj);
  assert.ok(str.includes('title = "TOML Example"'));
});

void test("encodes object to TOML", () => {
  const obj = { foo: "bar" };
  const encoded = encode(obj);
  assert.ok(encoded.includes('foo = "bar"'));
});
