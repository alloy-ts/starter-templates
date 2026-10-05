import { expect, test } from "vite-plus/test";
import { generateKeyPairSync, privateDecrypt, publicEncrypt } from "./main.ts";

test("generateKeyPairSync and RSA publicEncrypt / privateDecrypt", () => {
  const { publicKey, privateKey } = generateKeyPairSync("rsa", {
    modulusLength: 1024,
  });

  expect(publicKey).toContain("BEGIN PUBLIC KEY");
  expect(privateKey).toContain("BEGIN PRIVATE KEY");

  const plaintext = Buffer.from("Hello RSA encryption");
  const encrypted = publicEncrypt(publicKey, plaintext);
  expect(Buffer.isBuffer(encrypted)).toBe(true);

  const decrypted = privateDecrypt(privateKey, encrypted);
  expect(decrypted.toString()).toBe("Hello RSA encryption");
});
