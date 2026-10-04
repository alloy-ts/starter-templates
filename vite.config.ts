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
    options: { typeAware: true, typeCheck: true },
  },
  run: {
    cache: true,
  },
  tasks: {
    "ci:check": {
      command: "vp check",
      cache: {
        input: ["src/**", "Cargo.toml", "src/lib.rs", "package.json", "tsconfig.json"],
      },
    },
    "ci:build": {
      command: "node build.ts && vp pack",
      cache: {
        input: ["src/**", "Cargo.toml", "Cargo.lock", "src/lib.rs", "build.ts", "package.json"],
        output: ["dist/**", "*.node"],
      },
    },
    "ci:test": {
      command: "vp test",
      cache: {
        input: ["src/**", "dist/**", "*.node"],
      },
    },
    "ci:cross-build": {
      command: "node scripts/cross-build.ts",
      cache: {
        input: [
          "src/**",
          "Cargo.toml",
          "Cargo.lock",
          "src/lib.rs",
          "scripts/cross-build.ts",
          "package.json",
        ],
        output: ["dist/**", "*.node"],
      },
    },
    "ci:publish": {
      command: "npm publish --dry-run",
      cache: {
        input: ["dist/**", "package.json", "index.js", "index.d.ts", "ReadMe.md"],
      },
    },
    ci: {
      command: "node scripts/ci.ts",
      cache: false,
    },
  },
});
