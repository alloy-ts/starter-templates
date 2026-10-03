#!/usr/bin/env node

/**
 * Local CI Runner
 * Executes CI jobs locally without network/cloud dependencies or heavy microVMs.
 * Leverages native Node.js process management and Vite Task caching via @voidzero-dev/vite-task-client.
 */

import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";

// Simple YAML parser for basic .ci.yml structures without external dependency
function parseSimpleYaml(content) {
  const lines = content.split("\n");
  const jobSteps = {};
  let currentJob = null;
  let inSteps = false;

  for (let line of lines) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;

    const jobMatch = line.match(/^  ([a-zA-Z0-9_-]+):/);
    if (jobMatch && !line.startsWith("    ")) {
      currentJob = jobMatch[1];
      jobSteps[currentJob] = [];
      inSteps = false;
      continue;
    }

    if (line.includes("steps:")) {
      inSteps = true;
      continue;
    }

    if (currentJob && inSteps) {
      const match = trimmed.match(/^(?:-\s*)?run:\s*(.+)$/);
      if (match && match[1]) {
        jobSteps[currentJob].push(match[1].trim());
      }
    }
  }

  return jobSteps;
}

function runCommand(cmd, env = {}) {
  console.log(`\n> [LOCAL-CI] Executing: ${cmd}`);
  try {
    execSync(cmd, {
      stdio: "inherit",
      env: {
        ...process.env,
        CI: "true",
        ...env,
      },
    });
  } catch (err) {
    console.error(`\n[LOCAL-CI] Failed to execute command: ${cmd}`);
    process.exit(err.status || 1);
  }
}

async function main() {
  const args = process.argv.slice(2);
  let jobToRun = "all";

  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--job" && args[i + 1]) {
      jobToRun = args[i + 1];
      i++;
    }
  }

  const ciYamlPath = path.resolve(process.cwd(), ".ci.yml");
  let parsedJobs = {};

  if (fs.existsSync(ciYamlPath)) {
    const yamlContent = fs.readFileSync(ciYamlPath, "utf-8");
    parsedJobs = parseSimpleYaml(yamlContent);
  }

  console.log(`\n========================================`);
  console.log(`  Local CI Runner (Vite-Plus Orchestrated)`);
  console.log(`  Selected Job: ${jobToRun}`);
  console.log(`========================================`);

  const runJob = (jobName, defaultCmds) => {
    console.log(`\n--- Running Local CI Job: ${jobName} ---`);
    let cmds =
      parsedJobs[jobName] && parsedJobs[jobName].length > 0 ? parsedJobs[jobName] : defaultCmds;

    if (jobName === "cross") {
      const targets = [
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
        "x86_64-pc-windows-msvc",
        "aarch64-pc-windows-msvc",
        "x86_64-unknown-linux-gnu",
        "aarch64-unknown-linux-gnu",
        "x86_64-unknown-linux-musl",
        "aarch64-unknown-linux-musl",
        "wasm32-wasip1-threads",
      ];
      const expandedCmds = [];
      for (const cmd of cmds) {
        if (cmd.includes("${{ matrix.target }}")) {
          for (const target of targets) {
            expandedCmds.push(cmd.replace(/\$\{\{\s*matrix\.target\s*\}\}/g, target));
          }
        } else {
          expandedCmds.push(cmd);
        }
      }
      cmds = expandedCmds;
    }

    for (const cmd of cmds) {
      runCommand(cmd);
    }
  };

  if (jobToRun === "check" || jobToRun === "all") {
    runJob("check", ["vp check"]);
  }

  if (jobToRun === "build" || jobToRun === "all") {
    runJob("build", ["node build.js", "vp pack"]);
  }

  if (jobToRun === "test" || jobToRun === "all") {
    runJob("test", ["vp test", "node --test test.cjs"]);
  }

  if (jobToRun === "cross") {
    runJob("cross", [
      "node build.js --target x86_64-apple-darwin -x",
      "node build.js --target aarch64-apple-darwin -x",
      "node build.js --target x86_64-pc-windows-msvc -x",
      "node build.js --target x86_64-unknown-linux-gnu -x",
      "node build.js --target wasm32-wasip1-threads -x",
    ]);
  }

  if (jobToRun === "publish" || jobToRun === "all") {
    runJob("publish", ["npm publish --dry-run"]);
  }

  console.log(`\n✅ Local CI Job '${jobToRun}' completed successfully.`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
