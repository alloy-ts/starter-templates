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
    "*": ["vp check --fix", "vp run ci"],
  },
  fmt: {
    ignore: ["examples/**", "artifacts/**", "dist/**", "index.js", "index.d.ts"],
  },
  lint: {
    ignorePatterns: ["examples/**", "artifacts/**", "dist/**", "index.js", "index.d.ts"],
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: true, typeCheck: true },
  },
  run: {
    cache: true,
  },
  tasks: {
    ci: {
      command: "node scripts/ci-runner.js --job all",
      cache: {
        input: [{ auto: true }, "!dist/**", "!*.node"],
        output: ["dist/**", "*.node"],
      },
    },
    "ci:build": {
      command: "node scripts/ci-runner.js --job build",
      cache: {
        input: [{ auto: true }, "!dist/**", "!*.node"],
        output: ["dist/**", "*.node"],
      },
    },
    "ci:test": {
      command: "node scripts/ci-runner.js --job test",
      cache: {
        input: [{ auto: true }],
      },
    },
    "ci:cross": {
      command: "node scripts/ci-runner.js --job cross",
      cache: {
        input: [{ auto: true }, "!dist/**", "!*.node"],
        output: ["dist/**", "*.node"],
      },
    },
    "ci:publish": {
      command: "node scripts/ci-runner.js --job publish",
      cache: {
        input: [{ auto: true }],
      },
    },
  },
});
