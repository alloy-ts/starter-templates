import { expect, test } from "vite-plus/test";
import { createSign, generateKeyPairSync, sign, verify } from "./main.ts";

test("sign and verify with RSA key pair", () => {
  const { publicKey, privateKey } = generateKeyPairSync("rsa", {
    modulusLength: 1024,
  });

  const data = Buffer.from("data to sign");
  const signature = sign("sha256", data, privateKey);
  expect(Buffer.isBuffer(signature)).toBe(true);

  const isValid = verify("sha256", data, publicKey, signature);
  expect(isValid).toBe(true);
});

test("Sign class update and sign", () => {
  const { publicKey, privateKey } = generateKeyPairSync("rsa", {
    modulusLength: 1024,
  });

  const signer = createSign("sha256");
  signer.update("hello world");
  const sig = signer.sign(privateKey);
  expect(Buffer.isBuffer(sig)).toBe(true);

  const isValid = verify("sha256", "hello world", publicKey, sig);
  expect(isValid).toBe(true);
});
