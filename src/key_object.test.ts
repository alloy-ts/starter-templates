import { expect, test } from "vite-plus/test";
import { createSecretKey } from "../index.js";

test("createSecretKey creates KeyObject", () => {
  const secretBytes = Buffer.from("super-secret-key-1234");
  const keyObj = createSecretKey(secretBytes);
  expect(typeof keyObj.keyType).toBe("number");
  const exported = keyObj.export();
  expect(exported.toString("utf8")).toBe("super-secret-key-1234");
});
