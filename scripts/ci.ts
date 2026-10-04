import { execSync } from "node:child_process";
import process from "node:process";
import { getEnv } from "@voidzero-dev/vite-task-client";

const ciEnv = getEnv("CI") || "true";
process.env["CI"] = ciEnv;

function runStep(name: string, command: string) {
  console.log(`\n=== [CI] Running step: ${name} ===\n$ ${command}`);
  try {
    execSync(command, { stdio: "inherit" });
    console.log(`=== [CI] Completed step: ${name} ===\n`);
  } catch (error) {
    console.error(`=== [CI] Failed step: ${name} ===`);
    throw error;
  }
}

export function runCiCheck() {
  runStep(
    "Check (Lint, Format, Typecheck)",
    "npx vp check package.json vite.config.ts src/ scripts/ build.js build.mjs",
  );
}

export function runCiBuild() {
  runStep("Build (Host)", "node build.js --platform");
}

export function runCiCrossBuild() {
  runStep("Cross Build (All Targets)", "node build.js --target-all --use-cross");
}

export function runCiTest() {
  runStep("Test (Unit Tests)", "npx vp test src/main.test.ts");
}

export function runCiPublish() {
  runStep("Publish (Pack Dry Run)", "npx vp pack");
}

export function runAllCi() {
  console.log("=== Running Full Local CI Pipeline ===");
  runCiCheck();
  runCiBuild();
  runCiTest();
  runCiPublish();
  console.log("=== Local CI Pipeline Passed Successfully ===");
}

const args = process.argv.slice(2);
const jobIndex = args.indexOf("--job");
const job = jobIndex !== -1 ? args[jobIndex + 1] : undefined;

if (job === "check") {
  runCiCheck();
} else if (job === "build") {
  runCiBuild();
} else if (job === "cross-build") {
  runCiCrossBuild();
} else if (job === "test") {
  runCiTest();
} else if (job === "publish") {
  runCiPublish();
} else {
  runAllCi();
}
