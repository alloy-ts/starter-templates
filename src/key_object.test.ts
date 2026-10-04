import { expect, test } from "vite-plus/test";
import { createSecretKey, KeyType } from "../index.js";

test("createSecretKey creates KeyObject", () => {
  const secretBytes = Buffer.from("super-secret-key-1234");
  const keyObj = createSecretKey(secretBytes);
  expect(keyObj.keyType).toBe(KeyType.Secret);
  const exported = keyObj.export();
  expect(exported.toString("utf8")).toBe("super-secret-key-1234");
});
