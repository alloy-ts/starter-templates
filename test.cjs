const assert = require("node:assert/strict");
const test = require("node:test");

const { randomBytes } = require("./dist/index.js");

test("generates random bytes", () => {
  const buf = randomBytes(16);
  assert.equal(buf.length, 16);
});
