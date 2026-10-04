# Starter Template with Local CI & Cross-Build Pipeline

TypeScript & Native NAPI-RS starter template powered by Vite+ (`vp`).

## Local CI & Development Workflow

This template is configured to run all CI jobs locally without requiring external cloud/network services or heavy microVM containers.

### Local CI Commands

- **Run Full Local CI Job:**

  ```bash
  npm run ci
  # or using vp directly:
  vp run ci:pipeline
  ```

- **Run Native Build (with Task Caching):**

  ```bash
  npm run build
  # or
  vp run ci:build
  ```

- **Run Local Cross-Build Matrix:**

  ```bash
  npm run cross-build
  # or dry-run matrix:
  node scripts/cross-build.ts --dry-run
  ```

- **Run Unit Tests:**

  ```bash
  vp test
  ```

- **Format & Code Checks:**
  ```bash
  vp check
  vp check --fix
  ```

---

## Architecture & Features

### 1. Vite Task Runner & Task Caching (`@voidzero-dev/vite-task-client`)

CI tasks are configured in `vite.config.ts` under `run.tasks`:

- `ci:pipeline`: Runs `scripts/ci.ts` which orchestrates code check, unit tests, host native build, local cross-build matrix, and package packing.
- `ci:build`: Runs `scripts/build.ts` with automatic file system & cache output tracking (`dist/**`).
- `ci:cross-build`: Runs `scripts/cross-build.ts` across the target matrix.

When a task inputs have not changed, Vite Task replays cached terminal output and restores saved build outputs instantly.

### 2. Lightweight Native Execution vs Containers / MicroVMs

For cross-compiling native modules (such as NAPI-RS addons), the local CI pipeline uses:

- Node.js runtime scripts (`scripts/build.ts`, `scripts/cross-build.ts`, `scripts/ci.ts`) with `#!/usr/bin/env node` shebangs.
- Native cross-compilation toolchains (`cargo-zigbuild`, `cargo-xwin`, `@napi-rs/cli` `--use-napi-cross`, `-x`).
- WASM target execution (`wasm32-wasip1-threads`) directly via Node.js WASM runtime.

This avoids the overhead of Docker containers or microVMs while running cross-builds natively on your host OS.

### 3. Pre-Commit Hooks

Local checks are automatically triggered on staged files during git pre-commit:

```bash
npm run prepare # configures local hooks via vp config
```

The `staged` configuration in `vite.config.ts` automatically runs `vp check --fix` on staged files.

### 4. YAML Job Script Support

For compatibility with CI runners or GitHub Actions, `.ci/job.yml` and `.github/workflows/ci.yml` provide declarative YAML definitions using:

```yaml
- uses: voidzero-dev/setup-vp@v1
  with:
    node-version: "22"
    cache: true

- run: vp install
- run: vp check
- run: vp test
- run: vp run ci:pipeline
```
