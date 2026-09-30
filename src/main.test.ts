import { expect, test } from "vite-plus/test";
import { ObjectStore } from "../index.js";

test("ObjectStore memory test in typescript", async () => {
  const store = ObjectStore.memory();
  await store.put("ts/test.txt", Buffer.from("ts content"));
  const data = await store.getBytes("ts/test.txt");
  expect(data.toString()).toBe("ts content");
});
