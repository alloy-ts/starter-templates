import assert from "node:assert/strict";
import test from "node:test";
import { main } from "./main.ts";

test("main returns Hello, world!", () => {
  assert.equal(main(), "Hello, world!");
});
