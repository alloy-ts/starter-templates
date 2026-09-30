import { expect, test } from "vite-plus/test";
import Sqids, { defaultOptions } from "../index.js";
import { main } from "./main.ts";

test("main returns encoded ID 86Rf07", () => {
  expect(main()).toBe("86Rf07");
});

test("Sqids encodes and decodes array of numbers", () => {
  const sqids = new Sqids();
  const id = sqids.encode([1, 2, 3]);
  expect(id).toBe("86Rf07");
  expect(sqids.decode(id)).toEqual([1, 2, 3]);
});

test("Sqids with minLength", () => {
  const sqids = new Sqids({ minLength: 10 });
  const id = sqids.encode([1, 2, 3]);
  expect(id).toBe("86Rf07xd4z");
  expect(sqids.decode(id)).toEqual([1, 2, 3]);
});

test("Sqids with custom alphabet", () => {
  const sqids = new Sqids({
    alphabet: "FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE",
  });
  const id = sqids.encode([1, 2, 3]);
  expect(id).toBe("B4aajs");
  expect(sqids.decode(id)).toEqual([1, 2, 3]);
});

test("Sqids with blocklist Set", () => {
  const sqids = new Sqids({
    blocklist: new Set(["86Rf07"]),
  });
  const id = sqids.encode([1, 2, 3]);
  expect(id).toBe("se8ojk");
  expect(sqids.decode(id)).toEqual([1, 2, 3]);
});

test("defaultOptions export", () => {
  expect(defaultOptions.minLength).toBe(0);
  expect(defaultOptions.blocklist.has("0rgasm")).toBe(true);
});
