import { definePackageConfig } from "@alloy-ts/config";

export default definePackageConfig({
  name: "@lib/module",
  version: "0.0.1",
  type: "module",
  main: "src/main.ts",
  scripts: {
    prepare: "node node_modules/@alloy-ts/config/scripts/codegen.mjs",
    start: "node src/main.ts",
    test: "node --test",
  },
  dependencies: {},
  devDependencies: {
    "@alloy-ts/config": "git+https://github.com/alloy-ts/config.git#dist"
  }
});
