import { expect, test } from "vite-plus/test";
import { Tls } from "../index.js";

test("Tls class connects and disconnects using ring provider default", () => {
  const tls = new Tls();
  expect(tls.provider).toBe("ring");
  expect(tls.isConnected).toBe(false);

  const connected = tls.connect("example.com");
  expect(connected).toBe(true);
  expect(tls.isConnected).toBe(true);

  tls.disconnect();
  expect(tls.isConnected).toBe(false);
});
