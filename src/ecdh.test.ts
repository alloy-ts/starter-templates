import { expect, test } from "vite-plus/test";
import { createECDH, ECDH } from "./main.ts";

test("createECDH secp256r1 computeSecret", () => {
  const alice = createECDH("secp256r1");
  const bob = createECDH("secp256r1");

  alice.generateKeys();
  bob.generateKeys();

  const alicePub = alice.getPublicKey();
  const bobPub = bob.getPublicKey();

  const aliceSecret = alice.computeSecret(bobPub, undefined, "hex");
  const bobSecret = bob.computeSecret(alicePub, undefined, "hex");

  expect(aliceSecret).toBe(bobSecret);
});

test("ECDH class directly", () => {
  const alice = new ECDH("secp384r1");
  const keys = alice.generateKeys("hex");
  expect(typeof keys).toBe("string");
  expect(alice.getPrivateKey("hex")).toBeTruthy();
});
