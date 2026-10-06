import { expect, test } from "vite-plus/test";
import { CryptoKeyPair, KeyObject, X509Certificate } from "./main.ts";

test("KeyObject type, symmetricKeySize, asymmetricKeyType, and export", () => {
  const secretKey = new KeyObject("secret", Buffer.from("supersecret"), null);
  expect(secretKey.type).toBe("secret");
  expect(secretKey.symmetricKeySize).toBe(11);
  expect(secretKey.asymmetricKeyType).toBeFalsy();
  expect(secretKey.asymmetricKeyDetails).toBeFalsy();

  const exported = secretKey.export("utf8");
  expect(exported).toBe("supersecret");

  const pubKey = new KeyObject("public", Buffer.from("pubdata"), "ec");
  expect(pubKey.type).toBe("public");
  expect(pubKey.asymmetricKeyType).toBe("ec");
  expect(pubKey.symmetricKeySize).toBeFalsy();
  expect(pubKey.asymmetricKeyDetails?.namedCurve).toBe("prime256v1");
});

test("KeyObject equals and KeyObject.from", () => {
  const key1 = new KeyObject("secret", Buffer.from("secret123"), null);
  const key2 = new KeyObject("secret", Buffer.from("secret123"), null);
  const key3 = new KeyObject("secret", Buffer.from("different"), null);

  expect(key1.equals(key2)).toBe(true);
  expect(key1.equals(key3)).toBe(false);

  const keyFromStr = KeyObject.from("my-secret-key");
  expect(keyFromStr.type).toBe("secret");
  expect(keyFromStr.symmetricKeySize).toBe(13);
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
