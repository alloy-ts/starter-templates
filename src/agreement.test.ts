import { expect, test } from "vite-plus/test";
import {
  createMac,
  createPrivateKey,
  createPublicKey,
  createSecretKey,
  createVerify,
  decapsulate,
  encapsulate,
} from "./main.ts";

test("createPublicKey, createPrivateKey, createSecretKey", () => {
  const secret = createSecretKey(Buffer.from("rawsecret"));
  expect(secret.type).toBe("secret");

  const pub = createPublicKey(Buffer.from("pub"));
  expect(pub.type).toBe("public");

  const priv = createPrivateKey(Buffer.from("priv"));
  expect(priv.type).toBe("private");
});

test("encapsulate and decapsulate with X25519", () => {
  // 32-byte key for X25519
  const pub = Buffer.alloc(32, 0x01);
  const enc = encapsulate(pub);
  expect(Buffer.isBuffer(enc.sharedKey)).toBe(true);
  expect(enc.sharedKey.length).toBe(32);
  expect(Buffer.isBuffer(enc.ciphertext)).toBe(true);

  // Decapsulate using corresponding private key
  const priv = Buffer.alloc(32, 0x02);
  const dec = decapsulate(priv, enc.ciphertext);
  expect(Buffer.isBuffer(dec)).toBe(true);
  expect(dec.length).toBe(32);
});

test("createMac and createVerify", () => {
  const mac = createMac("sha256", "key");
  mac.update("data");
  const digest = mac.digest("hex");
  expect(typeof digest).toBe("string");

  const verifier = createVerify("sha256");
  verifier.update("data");
  const isValid = verifier.verify("key", Buffer.from(digest as string, "hex"));
  expect(typeof isValid).toBe("boolean");
});
