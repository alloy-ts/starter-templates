const assert = require("node:assert/strict");
const test = require("node:test");

const { createHash, hash } = require("./dist/index.js");

test("crypto hash", () => {
  const h = createHash("sha256");
  h.update("hello");
  assert.equal(h.digest("hex"), hash("sha256", "hello", "hex"));
});
