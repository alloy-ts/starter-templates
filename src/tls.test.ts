import { expect, test } from "vite-plus/test";
import { createTLS, TLS } from "./main.ts";

test("TLS class default ring provider", () => {
  const tls = new TLS("ring");
  expect(tls.provider).toBe("ring");
  expect(tls.isActive()).toBe(true);
  const cfg = tls.createClientConfig();
  expect(cfg).toContain("TLS ClientConfig created");
});

test("createTLS openssl provider", () => {
  const tls = createTLS("openssl");
  expect(tls.provider).toBe("openssl");
  expect(tls.isActive()).toBe(true);
});
