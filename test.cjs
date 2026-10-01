const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

void test("Repository native binding", async () => {
  const { Repository } = await import("./index.js");
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-test-cjs-"));
  try {
    const repo = Repository.init(tmpDir);
    assert.equal(repo.isBare(), false);
    assert.equal(repo.isEmpty(), true);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});
