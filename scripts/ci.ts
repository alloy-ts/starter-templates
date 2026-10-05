#!/usr/bin/env node

/**
 * Local TypeScript CI Pipeline
 * Runs local CI jobs natively with Vite Task Caching without external cloud CI or YML scripts.
 */

import { execSync } from "node:child_process";
import process from "node:process";

function runCommand(cmd: string, env: Record<string, string> = {}): void {
  console.log(`\n> [LOCAL-CI-TS] Executing: ${cmd}`);
  try {
    execSync(cmd, {
      stdio: "inherit",
      env: {
        ...process.env,
        CI: "true",
        ...env,
      },
    });
  } catch (err: unknown) {
    const error = err as { status?: number };
    console.error(`\n❌ [LOCAL-CI-TS] Failed to execute: ${cmd}`);
    process.exit(error.status || 1);
  }
}

async function main(): Promise<void> {
  const args = process.argv.slice(2);
  let jobToRun = "all";

  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--job" && args[i + 1]) {
      jobToRun = args[i + 1]!;
      i++;
    }
  }

  console.log(`\n========================================`);
  console.log(`  Local CI Pipeline (TypeScript)`);
  console.log(`  Selected Job: ${jobToRun}`);
  console.log(`========================================`);

  if (jobToRun === "check" || jobToRun === "all") {
    console.log(`\n--- Running Job: check ---`);
    runCommand("vp check");
  }

  if (jobToRun === "build" || jobToRun === "all") {
    console.log(`\n--- Running Job: build ---`);
    runCommand("vp pack");
    runCommand("node build.js");
  }

  if (jobToRun === "test" || jobToRun === "all") {
    console.log(`\n--- Running Job: test ---`);
    runCommand("vp test");
    runCommand("node --test test.cjs");
  }

  if (jobToRun === "cross" || jobToRun === "all") {
    console.log(`\n--- Running Job: cross ---`);
    runCommand("node build.js --target-all -x --dry-run");
  }

  if (jobToRun === "publish" || jobToRun === "all") {
    console.log(`\n--- Running Job: publish ---`);
    runCommand("npm publish --dry-run");
  }

  console.log(`\n✅ Local CI Job '${jobToRun}' completed successfully.`);
}

main().catch((err) => {
  console.error("Fatal error in local CI pipeline:", err);
  process.exit(1);
});
