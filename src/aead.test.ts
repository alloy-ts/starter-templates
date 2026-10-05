import { expect, test } from "vite-plus/test";
import { decryptAead, encryptAead } from "./main.ts";

test("encryptAead and decryptAead aes-128-gcm", () => {
  const key = Buffer.alloc(16, 0x01);
  const iv = Buffer.alloc(12, 0x02);
  const plaintext = Buffer.from("Secret payload data");

  const { ciphertext, authTag } = encryptAead("aes-128-gcm", key, iv, plaintext);
  expect(Buffer.isBuffer(ciphertext)).toBe(true);
  expect(Buffer.isBuffer(authTag)).toBe(true);

  const decrypted = decryptAead("aes-128-gcm", key, iv, ciphertext, authTag);
  expect(decrypted.toString()).toBe("Secret payload data");
});

test("encryptAead and decryptAead aes-128-siv", () => {
  const key = Buffer.alloc(32, 0x03);
  const iv = Buffer.alloc(16, 0x04);
  const plaintext = Buffer.from("SIV encrypted text");

  const { ciphertext, authTag } = encryptAead("aes-128-siv", key, iv, plaintext);
  const decrypted = decryptAead("aes-128-siv", key, iv, ciphertext, authTag);
  expect(decrypted.toString()).toBe("SIV encrypted text");
});

test("encryptAead and decryptAead aes-128-ccm", () => {
  const key = Buffer.alloc(16, 0x05);
  const iv = Buffer.alloc(12, 0x06);
  const plaintext = Buffer.from("CCM encrypted text");

  const { ciphertext, authTag } = encryptAead("aes-128-ccm", key, iv, plaintext);
  const decrypted = decryptAead("aes-128-ccm", key, iv, ciphertext, authTag);
  expect(decrypted.toString()).toBe("CCM encrypted text");
});
