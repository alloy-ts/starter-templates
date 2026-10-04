import { expect, test } from "vite-plus/test";
import { createSign, createVerify } from "../index.js";

test("Sign and Verify classes construct cleanly", () => {
  const sign = createSign("ed25519");
  sign.update(Buffer.from("message to sign"));
  expect(sign.algorithm).toBe("ed25519");

  const verify = createVerify("ed25519");
  verify.update(Buffer.from("message to sign"));
  expect(verify.algorithm).toBe("ed25519");
});
