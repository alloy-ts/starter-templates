import { defineConfig } from "vite-plus";

export default defineConfig({
  pack: {
    entry: ["./src/main.ts"],
    format: "esm",
    outDir: "dist",
    exports: true,
    dts: {
      generator: "oxc",
    },
    deps: {
      resolveDepSubpath: true,
    },
  },
  test: {
    include: ["src/**/*.{test,spec}.ts"],
  },
  staged: {
    "*": "vp check --fix",
  },
  fmt: {
    ignore: ["examples/**"],
  },
  lint: {
    ignorePatterns: ["examples/**"],
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
  },
  run: {
    cache: true,
    tasks: {
      "ci:pipeline": {
        command: "node scripts/ci.ts",
        cache: {
          env: ["CI", "NODE_ENV"],
        },
      },
      "ci:build": {
        command: "node scripts/build.ts",
        cache: {
          input: [{ auto: true }, "!dist/**"],
          output: ["dist/**"],
        },
      },
      "ci:cross-build": {
        command: "node scripts/cross-build.ts",
        cache: {
          input: [{ auto: true }, "!dist/**"],
          output: ["dist/**"],
        },
      },
      "ci:test": {
        command: "vp test",
      },
    },
  },
});
