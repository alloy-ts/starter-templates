const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

void test("Repository native binding", async () => {
  const { ReferenceType, Repository, RepositoryState } = await import("./index.js");
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-test-cjs-"));
  try {
    const repo = Repository.init(tmpDir);
    assert.equal(repo.isBare(), false);
    assert.equal(repo.isEmpty(), true);
    assert.equal(repo.stateEnum(), RepositoryState.Clean);

    const ref = repo.createSymbolicReference("HEAD", "refs/heads/main", true, "init HEAD");
    assert.equal(ref.name(), "HEAD");
    assert.equal(ref.symbolicTarget(), "refs/heads/main");
    assert.equal(ref.kind(), ReferenceType.Symbolic);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});
