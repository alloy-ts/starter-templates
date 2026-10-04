#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { NapiCli } from "@napi-rs/cli";

interface TargetMatrixEntry {
  target: string;
  flags: string;
}

interface ParsedArgs {
  targetFilter: string | null;
  release: boolean;
  dryRun: boolean;
  buildAll: boolean;
  useNapiCross: boolean;
  crossCompile: boolean;
  useCross: boolean;
}

function getTargetMatrix(): TargetMatrixEntry[] {
  const pkgPath = path.resolve(process.cwd(), "package.json");
  let targets: string[] = [];

  if (fs.existsSync(pkgPath)) {
    try {
      const pkgContent = fs.readFileSync(pkgPath, "utf-8");
      const pkgJson = JSON.parse(pkgContent);
      if (pkgJson?.napi?.targets && Array.isArray(pkgJson.napi.targets)) {
        targets = pkgJson.napi.targets;
      }
    } catch (err) {
      console.warn(
        "Could not read targets from package.json, falling back to default matrix.",
        err,
      );
    }
  }

  if (targets.length === 0) {
    targets = [
      "x86_64-apple-darwin",
      "aarch64-apple-darwin",
      "x86_64-pc-windows-msvc",
      "i686-pc-windows-msvc",
      "aarch64-pc-windows-msvc",
      "x86_64-unknown-linux-gnu",
      "aarch64-unknown-linux-gnu",
      "x86_64-unknown-linux-musl",
      "aarch64-unknown-linux-musl",
      "armv7-unknown-linux-gnueabihf",
      "powerpc64le-unknown-linux-gnu",
      "s390x-unknown-linux-gnu",
      "wasm32-wasip1-threads",
    ];
  }

  return targets.map((target) => ({
    target,
    flags:
      target.includes("linux-gnu") ||
      target.includes("gnueabihf") ||
      target.includes("powerpc") ||
      target.includes("s390x")
        ? "--use-napi-cross"
        : "-x",
  }));
}

function parseArgs(): ParsedArgs {
  const args = process.argv.slice(2);

  let targetFilter: string | null = null;
  const targetIdx = args.findIndex((a) => a === "--target" || a === "-t");
  const targetVal = targetIdx !== -1 ? args[targetIdx + 1] : undefined;
  if (targetVal && !targetVal.startsWith("-")) {
    targetFilter = targetVal;
  }

  const release = args.includes("--release") || args.includes("-r");
  const dryRun = args.includes("--dry-run");
  const buildAll = args.includes("--all") || args.includes("--build-all") || !targetFilter;

  const useNapiCross = args.includes("--use-napi-cross");
  const crossCompile = args.includes("--cross-compile") || args.includes("-x");
  const useCross = args.includes("--use-cross");

  return {
    targetFilter,
    release,
    dryRun,
    buildAll,
    useNapiCross,
    crossCompile,
    useCross,
  };
}

async function runCrossBuild() {
  const TARGET_MATRIX = getTargetMatrix();
  const { targetFilter, release, dryRun, buildAll, useNapiCross, crossCompile, useCross } =
    parseArgs();

  const cli = new NapiCli();
  let targetsToBuild: Array<{ target: string | undefined; flags: string }> = [];

  if (targetFilter) {
    const match = TARGET_MATRIX.find((t) => t.target === targetFilter);
    targetsToBuild = match ? [match] : [{ target: targetFilter, flags: "-x" }];
  } else if (buildAll) {
    targetsToBuild = TARGET_MATRIX;
  } else {
    targetsToBuild = [{ target: undefined, flags: "" }];
  }

  console.log(`\n🚀 Starting Local Cross-Build Pipeline (${targetsToBuild.length} target(s))...`);

  for (const { target, flags } of targetsToBuild) {
    const targetUseNapiCross = useNapiCross || flags.includes("--use-napi-cross");
    const targetCrossCompile =
      crossCompile || flags.includes("-x") || flags.includes("--cross-compile");
    const targetUseCross = useCross || flags.includes("--use-cross");

    const activeFlagsStr = [
      targetUseNapiCross ? "--use-napi-cross" : "",
      targetCrossCompile ? "-x" : "",
      targetUseCross ? "--use-cross" : "",
    ]
      .filter(Boolean)
      .join(" ");

    const cmd =
      `node scripts/build.ts ${target ? `--target ${target}` : ""} ${activeFlagsStr} ${release ? "--release" : ""}`.trim();
    console.log(`\n⚙️  Building target: ${target ?? "host"}`);
    console.log(`   Command: ${cmd}`);

    if (dryRun) {
      console.log(`   [Dry Run] Skipped execution.`);
      continue;
    }

    try {
      const res = await cli.build({
        platform: true,
        esm: true,
        outputDir: "./dist",
        release,
        target,
        useNapiCross: targetUseNapiCross,
        crossCompile: targetCrossCompile,
        useCross: targetUseCross,
      } as any);
      if (res && typeof (res as any)["task"] !== "undefined") {
        await (res as any)["task"];
      }
      console.log(`✅ Target ${target ?? "host"} built successfully.`);
    } catch (err: any) {
      console.error(`❌ Target ${target ?? "host"} failed to build:`, err?.message || err);
      if (!buildAll) {
        process.exit(1);
      }
    }
  }

  console.log(`\n✨ Cross-build process complete!`);
}

runCrossBuild().catch((err) => {
  console.error("Fatal error during cross-build:", err);
  process.exit(1);
});
