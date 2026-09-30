import { expect, test } from "vite-plus/test";
import { parse, stringify, decode, encode } from "./main.ts";

test("parse and stringify TOML", () => {
  const obj = { name: "toml", version: "1.0" };
  const str = stringify(obj);
  const parsed = parse(str);
  expect(parsed).toEqual(obj);
});

test("encode and decode TOML", () => {
  const obj = { key: "value" };
  const str = encode(obj);
  const decoded = decode(str);
  expect(decoded).toEqual(obj);
});
