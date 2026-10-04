#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { createBuildCommand, NapiCli } from "@napi-rs/cli";

const pkgPath = path.resolve(process.cwd(), "package.json");
const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
const pkgNapiTargets: string[] = pkg.napi?.targets || [
  "x86_64-apple-darwin",
  "aarch64-apple-darwin",
  "x86_64-pc-windows-msvc",
  "i686-pc-windows-msvc",
  "aarch64-pc-windows-msvc",
  "x86_64-unknown-linux-gnu",
  "aarch64-unknown-linux-gnu",
  "armv7-unknown-linux-gnueabihf",
  "x86_64-unknown-linux-musl",
  "aarch64-unknown-linux-musl",
  "powerpc64le-unknown-linux-gnu",
  "s390x-unknown-linux-gnu",
];

function parseArgs() {
  const args = process.argv.slice(2);

  const buildAll = args.includes("--target-all") || args.includes("--all");
  const isRelease = args.includes("--release") || args.includes("-r");
  const dryRun = args.includes("--dry-run");

  const useCross = args.includes("--use-cross");
  const useNapiCross = args.includes("--use-napi-cross");
  const crossCompile = args.includes("-x") || args.includes("--cross-compile");

  let targetFilter: string | null = null;
  const targetIdx = args.findIndex((a) => a === "--target" || a === "-t");
  const targetVal = targetIdx !== -1 ? args[targetIdx + 1] : undefined;
  if (targetVal && !targetVal.startsWith("-")) {
    targetFilter = targetVal;
  }

  const filteredArgs = args.filter(
    (arg) => arg !== "--target-all" && arg !== "--all" && arg !== "--dry-run",
  );

  return {
    targetFilter,
    isRelease,
    dryRun,
    buildAll,
    useCross,
    useNapiCross,
    crossCompile,
    filteredArgs,
  };
}

async function runBuild() {
  const {
    targetFilter,
    isRelease,
    dryRun,
    buildAll,
    useCross,
    useNapiCross,
    crossCompile,
    filteredArgs,
  } = parseArgs();

  const buildCommand = createBuildCommand(filteredArgs);
  const options = buildCommand.getOptions();
  const cli = new NapiCli();

  let targetsToBuild: (string | null)[] = [];

  if (targetFilter) {
    targetsToBuild = [targetFilter];
  } else if (buildAll) {
    targetsToBuild = pkgNapiTargets;
  } else {
    targetsToBuild = [null];
  }

  console.log(`\n🚀 Starting Local Build Pipeline (${targetsToBuild.length} target(s))...`);

  for (const target of targetsToBuild) {
    const isNapiCrossTarget =
      target !== null &&
      (target.includes("gnueabihf") ||
        target.includes("powerpc") ||
        target.includes("s390x") ||
        target.includes("linux-gnu"));
    const effectiveUseNapiCross = useNapiCross || (target ? isNapiCrossTarget : false);
    const effectiveCrossCompile = crossCompile || (target ? !isNapiCrossTarget : false);

    console.log(`\n⚙️  Building target: ${target || "default host"}`);
    if (dryRun) {
      console.log(`   [Dry Run] Skipped build execution for ${target || "default host"}.`);
      continue;
    }

    try {
      const buildOpts: Record<string, unknown> = {
        ...options,
        cwd: options.cwd || process.cwd(),
        outputDir: options.outputDir || "./dist",
        release: isRelease,
        cargoOptions: buildCommand.cargoOptions,
      };

      if (target) {
        buildOpts["target"] = target;
      }
      if (useCross) buildOpts["useCross"] = true;
      if (effectiveUseNapiCross) buildOpts["useNapiCross"] = true;
      if (effectiveCrossCompile) buildOpts["crossCompile"] = true;

      const res = await cli.build(buildOpts as any);
      if (res && typeof (res as any)["task"] !== "undefined") {
        await (res as any)["task"];
      }
      console.log(`✅ Target ${target || "default host"} built successfully.`);
    } catch (err: any) {
      console.error(`❌ Target ${target || "default host"} failed to build:`, err?.message || err);
      if (!buildAll) {
        process.exit(1);
      }
    }
  }

  console.log(`\n✨ Build process complete!`);
}

runBuild().catch((err) => {
  console.error("Fatal error during build:", err);
  process.exit(1);
});
