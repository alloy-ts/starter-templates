import { defineConfig } from "vite-plus";

export default defineConfig({
  pack: {
    entry: ["./src/main.ts"],
    format: "esm",
    outDir: "dist",
    exports: true,
    dts: {
      generator: "tsgo",
    },
    deps: {
      resolveDepSubpath: true,
    },
  },
  staged: {
    "*": "vp check --fix package.json vite.config.ts src/ scripts/ build.js build.mjs",
  },
  fmt: {},
  lint: {
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: true, typeCheck: true },
  },
  run: {
    cache: {
      tasks: true,
      scripts: false,
    },
  },
  tasks: {
    "task:ci:check": {
      command: "vp check package.json vite.config.ts src/ scripts/ build.js build.mjs",
    },
    "task:ci:build": {
      command: "node build.js --platform",
      cache: {
        input: [{ auto: true }, "!dist/**", "!target/**"],
        output: ["*.node", "artifacts/**"],
      },
    },
    "task:ci:test": {
      command: "vp test src/*.test.ts",
    },
    "task:ci:cross-build": {
      command: "node scripts/ci.ts --job cross-build",
    },
    "task:ci:publish": {
      command: "vp pack",
      cache: {
        input: [{ auto: true }],
        output: ["dist/**"],
      },
    },
    "task:ci:pipeline": {
      command: "node scripts/ci.ts",
    },
  },
});
