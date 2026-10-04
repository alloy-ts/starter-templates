import { expect, test } from "vite-plus/test";
import { createHmac } from "../index.js";

test("Hmac creates valid sha256 digest", () => {
  const key = Buffer.from("secret-key");
  const hmac = createHmac("sha256", key);
  hmac.update(Buffer.from("hello world"));
  const digestHex = hmac.digest("hex");
  expect(typeof digestHex).toBe("string");
  expect(digestHex.length).toBe(64);
});
