import { expect, test } from "vite-plus/test";
import { CryptoKeyPair, KeyObject, X509Certificate } from "./main.ts";

test("KeyObject type and export", () => {
  const secretKey = new KeyObject("secret", Buffer.from("supersecret"), null);
  expect(secretKey.type).toBe("secret");
  const exported = secretKey.export("utf8");
  expect(exported).toBe("supersecret");
});

test("CryptoKeyPair class", () => {
  const pub = new KeyObject("public", Buffer.from("pubdata"), "rsa");
  const priv = new KeyObject("private", Buffer.from("privdata"), "rsa");
  const pair = new CryptoKeyPair(pub, priv);

  expect(pair.publicKey.type).toBe("public");
  expect(pair.privateKey.type).toBe("private");
});

test("X509Certificate properties and methods", () => {
  const cert = new X509Certificate("dummy cert data");
  expect(cert.ca).toBe(false);
  expect(typeof cert.fingerprint).toBe("string");
  expect(typeof cert.fingerprint256).toBe("string");
  expect(cert.checkEmail("test@example.com")).toBe(false);
});
