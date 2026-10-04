#!/usr/bin/env node

import { execSync } from "node:child_process";
import process from "node:process";

function runStep(name: string, command: string) {
  console.log(`\n========================================`);
  console.log(`🏃 Running CI Step: ${name}`);
  console.log(`   Command: ${command}`);
  console.log(`========================================\n`);

  try {
    execSync(command, { stdio: "inherit", env: process.env });
    console.log(`\n✅ Step '${name}' completed successfully.`);
  } catch (error) {
    console.error(`\n❌ Step '${name}' failed.`);
    throw error;
  }
}

async function main() {
  console.log("🚀 Starting Local CI Pipeline...\n");

  const isQuick = process.argv.includes("--quick");
  const isDryRun = process.argv.includes("--dry-run");

  // 1. Code Checks
  runStep("Code Quality & Linting", "npx vp check");

  // 2. Local Build
  runStep("Build Native & JS Outputs", "npx vp run ci:build");

  // 3. Test Execution
  runStep("Run Tests", "npx vp test");

  // 4. Cross-Build Check
  if (!isQuick) {
    const crossBuildCmd = isDryRun
      ? "node scripts/cross-build.ts --dry-run"
      : "node scripts/cross-build.ts";
    runStep("Cross-Build Targets Check", crossBuildCmd);
  }

  // 5. Package Publish Verification
  runStep("Dry-Run Package Publish", "npm publish --dry-run");

  console.log("\n🎉 All Local CI Jobs Passed Successfully!");
}

main().catch((err) => {
  console.error("\n💥 CI Pipeline Failed:", err?.message || err);
  process.exit(1);
});
