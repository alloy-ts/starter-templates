import { expect, test } from "vite-plus/test";
import { createHmac, Hmac } from "./main.ts";

test("createHmac sha256 hex digest", () => {
  const hmac = createHmac("sha256", "secret_key");
  hmac.update("message");
  const digest = hmac.digest("hex");
  expect(typeof digest).toBe("string");
  expect(digest.length).toBe(64);
});

test("Hmac class directly", () => {
  const hmac = new Hmac("sha512", "secret_key");
  hmac.update("another message");
  const digest = hmac.digest("hex");
  expect(typeof digest).toBe("string");
  expect(digest.length).toBe(128);
});
