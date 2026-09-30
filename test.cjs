const assert = require("node:assert/strict");
const test = require("node:test");

void test("Diesel ORM CJS bindings", async () => {
  const { establishConnection } = await import("./index.js");
  const conn = establishConnection(":memory:");
  conn.setupTables();

  const user = conn.createUsers([{ name: "Alice", hairColor: "Blonde" }])[0];
  assert.equal(user.name, "Alice");

  const users = conn.getUsers();
  assert.equal(users.length, 1);
});
