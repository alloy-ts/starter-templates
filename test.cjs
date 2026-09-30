const assert = require("node:assert/strict");
const test = require("node:test");
const path = require("node:path");
const os = require("node:os");
const fs = require("node:fs/promises");

const { ObjectStore } = require("./index.js");

test("in-memory object store put, get, head, list, copy, rename, delete, getRanges", async () => {
  const store = ObjectStore.memory();

  // put
  await store.put("foo/bar.txt", Buffer.from("hello world"));

  // get
  const getRes = await store.get("foo/bar.txt");
  assert.equal(getRes.bytes.toString(), "hello world");
  assert.equal(getRes.meta.location, "foo/bar.txt");
  assert.equal(getRes.meta.size, 11);

  // getBytes
  const bytes = await store.getBytes("foo/bar.txt");
  assert.equal(bytes.toString(), "hello world");

  // head
  const headRes = await store.head("foo/bar.txt");
  assert.equal(headRes.location, "foo/bar.txt");
  assert.equal(headRes.size, 11);

  // list
  const listRes = await store.list("foo");
  assert.equal(listRes.length, 1);
  assert.equal(listRes[0].location, "foo/bar.txt");

  // getRanges
  const ranges = await store.getRanges("foo/bar.txt", [
    { start: 0, end: 5 },
    { start: 6, end: 11 },
  ]);
  assert.equal(ranges.length, 2);
  assert.equal(ranges[0].toString(), "hello");
  assert.equal(ranges[1].toString(), "world");

  // copy
  await store.copy("foo/bar.txt", "foo/copied.txt");
  const copiedBytes = await store.getBytes("foo/copied.txt");
  assert.equal(copiedBytes.toString(), "hello world");

  // rename
  await store.rename("foo/copied.txt", "foo/renamed.txt");
  const renamedBytes = await store.getBytes("foo/renamed.txt");
  assert.equal(renamedBytes.toString(), "hello world");

  // delete
  await store.delete("foo/bar.txt");
  await store.delete("foo/renamed.txt");

  const listAfterDelete = await store.list("foo");
  assert.equal(listAfterDelete.length, 0);
});

test("local file system object store", async () => {
  const tmpDir = await fs.mkdtemp(path.join(os.tmpdir(), "object-store-test-"));
  try {
    const store = ObjectStore.local(tmpDir);

    await store.put("test.txt", Buffer.from("local fs test"));
    const data = await store.getBytes("test.txt");
    assert.equal(data.toString(), "local fs test");

    const meta = await store.head("test.txt");
    assert.equal(meta.size, 13);
  } finally {
    await fs.rm(tmpDir, { recursive: true, force: true });
  }
});
